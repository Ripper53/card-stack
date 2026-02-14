# Card Game
A TCG framework for TCG frameworks.

**NOTE:** crate is in-production. This is a one man project. The documentation is sparse. The crate in its current state can be used to create a card game, but there is still more work left to do. More features are yet to be added and the workflow made more ergonomic. Now, back to work...

**When using this crate, make sure to also use the `state-validation` crate!**

What problems does this crate solve?
1. The Stack
2. Resolving The Stack so actions can be taken
3. What if multiple actions want to occur at the same time?
4. I want an ability listening for when something happens!

There are a lot of other problems that are solved,
or give you an API layer to solve the problems.

Now, what is important to know?
1. This crate tries to solve problems of TCGs, and is used in conjunction with the `state-validation` crate.
2. `state-validation` is used to solve the conditions for when an ability
should trigger or if a card can be played, ect...
3. You will mostly be implementing the `StateFilter` and `ValidAction` traits.
4. Knowing the distinct states of your game is important. (We'll get to that)
5. Defining an event manager is important. (We'll get to that)

Here is an example of an extremely useful filter:
```rust
/// Validates card is within the hand zone.
//      (State of Game)                        (Input for Filter using newtype)
impl<State: GetState<Game>> StateFilter<State, (PlayerID, CardID)> for CardIn<HandZone> {
    type ValidOutput = (ValidPlayerID<()>, ValidCardID<Self>);
    type Error = std::convert::Infallible;
    fn filter(
        state: &State,
        (player_id, card_id): (PlayerID, CardID),
    ) -> Result<Self::ValidOutput, Self::Error> {
        let state = state.state();
        let valid_player_id = ValidPlayerID::try_new(&state.player_manager, player_id).unwrap();
        let valid_card_id = ValidCardID::try_new(card_id, &state.hand_zone).unwrap();
        Ok((valid_player_id, valid_card_id))
    }
}
```

Then, you can create an action that uses the filter.
```rust
struct PlayCard;
impl<State: GetState<Game>> ValidAction<State, (PlayerID, CardID)> for PlayCard {
    // Filter used here!
    type Filter = CardIn<HandZone>;
    type Output = State;
    fn with_valid_input(
        state: State,
        valid: <Self::Filter as StateFilter<State, (PlayerID, CardID)>>::ValidOutput,
    ) -> Self::Output {
        todo!()
    }
}
```
And finally run it:
```rust
let _ = match Validator::try_new(game, (player_id, card_id)) {
    Ok(context) => context.execute(PlayCard),
    Err(_) => todo!(),
};
```
`Ok` is returned if the action can be executed, otherwise `Err`.

Notice how we are using generics for the `State` in the implementations.
The reason is because our game may have multiple states.
For example, a main phase and a combat phase.
We may want some filters or actions to be only defined for certain states of the game.

Imagine a card with such an ability: "during the main phase, you may draw one card, once per turn."
We know this ability can only activate during the main phase,
so when we choose to implement its `ValidAction`,
we may choose to implement it for a specific state.
```rust
struct DrawCardDuringMainPhaseAbility;
impl ValidAction<MainPhase, ()> for DrawCardDuringMainPhaseAbility {
    type Filter = CardIn<HandZone>;
    type Output = State;
    fn with_valid_input(
        state: MainPhase,
        valid: <Self::Filter as StateFilter<MainPhase, ()>>::ValidOutput,
    ) -> Self::Output {
        todo!()
    }
}

struct MainPhase(Game);
impl GetState<Game> for MainPhase {
    fn state(&self) -> &Game {
        &self.0
    }
}
```
The generic implementation of `CardIn<HandZone>` becomes important here. Since internally, `MainPhase` contains the `Game`, and we implement `GetState<Game>` for `MainPhase`, it will work properly.

## Event Manager
An event manager is in charge of letting cards listen for events so they may trigger their abilities.

You must create a custom event manager with this handy macro:
```rust
#[card_game::event_manager(
    states = (
        |main_phase: &MainPhase| main_phase.game().card_manager().event_manager(),
        |combat_phase: &CombatPhase| combat_phase.game().card_manager().event_manager(),
    ) as T,
    events = (
        on_play -> Play ^ PlayStackable => PlayResolution,
        on_death -> Death ^ DeathStackable => DeathResolution,
    ),
)]
struct EventManager {}
```
That looks complicated! What does it describe?

The first field `states`, is a tuple of closures that only take the state as a parameter.
The closure itself returns the event manager that we are using the macro on.
For this example, we have only two states. The `MainPhase` and `CombatPhase`.
After the tuple, we have the syntax `as T`. We will get to what that means later.

For now, let's move onto what the `events` field is.
The `events` field is a tuple of events that can be triggered using `TriggeredEvent::new`.
For this example, we have when a card is played and when a card dies.

The first part: `on_play` is the function name that lets an event listener listen to this event for **all** states (`MainPhase`, `CombatPhase`).

The second part: `Play` is the event. This is the event you choose that will be triggered. In this example, let's assume this is `Play`'s definition:
```rust
#[derive(Clone)]
struct Play {
    player_id: PlayerID,
    card_id: CardID,
}
```
The third part: `PlayStackable` is an enum that is generated automatically.
It contains all the actions that can be stacked on The Stack when a `Play` event triggers.

The fourth part: `PlayResolution` is an enum that every stacked action must resolve into.
Usually, `PlayResolution` is not directly returned from such actions,
rather something that implements `Into<PlayResolution>` would be.
What can be resolved into by default?
The state itself or a `TriggeredEvent` using that state.

The generated stackable and resolutions should suffice for most games,
but in case you want something particular to be stacked or resolved into,
you can use this syntax:
```rust
on_play -> Play ^ PlayStackable {
    custom_stackable_item: usize,
} => PlayResolution {
    custom_resolution_item: f32,
},
```
This will allow `usize` to be stacked on the stack and `f32` to be resolved into.
These types are meaningless here, but you can use this for any meaningful types.

Remember, the only thing you need to define is the event data structure, in this case `Play`. The rest is taken care of.


Next, you will need to implement a `Resolver`.
To do this, you will need to create a type that implements `StackResolver` for every state.
Before we can do that, we really need to understand the relationship between our state and The Stack.

## What does "state" mean in relation to The Stack?
If you have a `state`, it can be represented by `Priority<State>`.

If you have a state with an active stack, it can be represented by `PriorityStack<State, IncitingAction>`.

An important thing to note is, that the inciting action (the first action on the stack) determines what can be stacked further on it.
Usually, you won't have to worry about that because the `event_manager` macro takes care of that for you.
But, it is still important to understand. If you look at `PriorityStack<State, IncitingAction>` now,
you can see that this is the state where there is an active stack. Meaning it is time to either
put actions onto The Stack or resolve them. Only when The Stack is fully resolved (or canceled) can the game take any other actions.
Because of this, every state has its sibling, the "stack state" represented with `PriorityStack<State, IncitingAction>`.

You will see `Priority<State>` if it is simply the state itself, with *no* active stack.

You will see `Priority<State, IncitingAction>` if it is the state *with* an active stack.

## Resolver?
Now, let's go back to how we should implement `StackResolver`.

There are two traits, the first is `IncitingResolver` which resolves an inciting action (the first action on the stack).

Then, there is the second trait `StackResolver` which resolves any actions stacked on the inciting action.

Usually, you will only need to implement `StackResolver`. But, because of what we learned before, you must do this for every state. That includes states with and without a stack. So, your implementations will look something akin to:
```
struct Resolver;
impl
    StackResolver<
        MainPhase,
        EventAction<
            Priority<MainPhase>,
            Play,
            PlayResolution<Priority<MainPhase>>,
        >,
    > for Resolver { ... }
```
Now, hold on! What is an `EventAction`?

We are implementing the stack resolver for the `MainPhase`,
where the inciting action is an `EventAction`.
In this case, it's when the `EventAction` which was triggered,
was triggered during the `Priority<MainPhase>` state,
by the `Play` event, with the resolution of `PlayResolution<State>`
where `State` is the state.

Note, every `EventAction` that is an inciting action,
will always have its resolution state be `Priority<State>`
since once it is resolved, there is no longer an active stack.

## Event Listeners!
Back to the event manager for a bit. How do we create abilities that can listen for certain events?

`EventListenerConstructor` and `EventListener` traits are what you are looking for.

## Card Builders let you look up events
Use the `CardBuilder` which can be retrieved from the `CardManager` to build your cards and listen for events. Then, simply add them to a zone within your game.

## What holds cards? Answer: Zones
Here are the traits you need to implement:
1. `Zone`

If you want the zone to be indexed:

2. `ArrayZone` (optional)

Now, do you want the zone to be finite or infinite?

3. `FiniteZone` or `InfiniteZone`

## What manages cards and their abilities?
The `CardManager<EventManager>` where `EventManager` is the custom event manager you defined.

You will also need a `PlayerManager`, and a `ZoneManager<Zones>`, where `Zones` is a data structure that contains the zones for each player.

---

This is a TCG framework for your own TCG framework. Use these tools to build an API layer for your TCG.
