use std::{collections::HashMap, hash::Hash};

use crate::{
    cards::CardID,
    create_valid_identification,
    identifications::SourceCardID,
    validation::{StateFilter, StateFilterInput, ValidAction},
};

pub struct EventManager<State, E: Event<State>, Listener: EventListener<State, E>> {
    events: Vec<Listener>,
    _m: std::marker::PhantomData<(State, E)>,
}
pub struct SimultaneousActionManager<State, E: Event<State>, Listener: EventListener<State, E>> {
    state: State,
    actions: Vec<SimultaneousAction<Listener::Action>>,
    valid_input: <<<Listener as EventListener<State, E>>::Action as ValidAction<
        State,
        <<Listener as EventListener<State, E>>::Filter as StateFilter<
            State,
            <E as Event<State>>::Input,
        >>::ValidOutput,
    >>::Filter as StateFilter<
        State,
        <<Listener as EventListener<State, E>>::Filter as StateFilter<
            State,
            <E as Event<State>>::Input,
        >>::ValidOutput,
    >>::ValidOutput,
    _m: std::marker::PhantomData<(E, Listener)>,
}
pub enum SimultaneousAction<Action> {
    Unresolved(Action),
    Resolved,
    Fizzled,
}
impl<Action> SimultaneousAction<Action> {
    fn resolve(&mut self) -> Action {
        let action = std::mem::replace(self, SimultaneousAction::Resolved);
        if let SimultaneousAction::Unresolved(action) = action {
            action
        } else {
            unreachable!();
        }
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct SimultaneousActionID(usize);
use crate as card_game;
create_valid_identification!(ValidSimultaneousActionID, SimultaneousActionID);
impl<F> ValidSimultaneousActionID<F> {
    pub(crate) fn new(id: SimultaneousActionID) -> Self {
        ValidSimultaneousActionID(id, std::marker::PhantomData::default())
    }
}

impl<State, E: Event<State>, Listener: EventListener<State, E>> EventManager<State, E, Listener> {
    pub fn new() -> Self {
        EventManager {
            events: Vec::new(),
            _m: std::marker::PhantomData::default(),
        }
    }
    pub fn add_listener(&mut self, listener: Listener) {
        self.events.push(listener);
    }
    pub(crate) fn collect_actions(
        &self,
        event: &E,
        valid_input: <<<Listener as EventListener<State, E>>::Action as ValidAction<
            State,
            <<Listener as EventListener<State, E>>::Filter as StateFilter<
                State,
                <E as Event<State>>::Input,
            >>::ValidOutput,
        >>::Filter as StateFilter<
            State,
            <<Listener as EventListener<State, E>>::Filter as StateFilter<
                State,
                <E as Event<State>>::Input,
            >>::ValidOutput,
        >>::ValidOutput,
    ) -> CollectedActions<State, E, Listener> {
        let actions = self
            .events
            .iter()
            .map(|listener| listener.action(event))
            .collect();
        CollectedActions {
            valid_input,
            actions,
            _m: std::marker::PhantomData::default(),
        }
    }
}
struct CollectedActions<State, E: Event<State>, Listener: EventListener<State, E>> {
    actions: Vec<Listener::Action>,
    valid_input: <<<Listener as EventListener<State, E>>::Action as ValidAction<
        State,
        <<Listener as EventListener<State, E>>::Filter as StateFilter<
            State,
            <E as Event<State>>::Input,
        >>::ValidOutput,
    >>::Filter as StateFilter<
        State,
        <<Listener as EventListener<State, E>>::Filter as StateFilter<
            State,
            <E as Event<State>>::Input,
        >>::ValidOutput,
    >>::ValidOutput,
    _m: std::marker::PhantomData<(State, E, Listener)>,
}
impl<State, E: Event<State>, Listener: EventListener<State, E>>
    CollectedActions<State, E, Listener>
{
    pub fn simultaneous_action_manager(
        self,
        state: State,
    ) -> SimultaneousActionManager<State, E, Listener> {
        SimultaneousActionManager {
            state,
            actions: self
                .actions
                .into_iter()
                .map(|action| SimultaneousAction::Unresolved(action))
                .collect(),
            valid_input: self.valid_input,
            _m: std::marker::PhantomData::default(),
        }
    }
}
impl<State, E: Event<State>, Listener: EventListener<State, E>>
    SimultaneousActionManager<State, E, Listener>
{
    pub fn simultaneous_action_ids(&self) -> impl Iterator<Item = ValidSimultaneousActionID<()>> {
        self.actions
            .iter()
            .enumerate()
            .map(|(index, _action)| ValidSimultaneousActionID::new(SimultaneousActionID(index)))
    }
    pub fn resolve<F>(
        mut self,
        action_id: ValidSimultaneousActionID<F>,
    ) -> <<Listener as EventListener<State, E>>::Action as ValidAction<
        State,
        <<Listener as EventListener<State, E>>::Filter as StateFilter<
            State,
            <E as Event<State>>::Input,
        >>::ValidOutput,
    >>::Output {
        let action = self.actions.get_mut(action_id.0.0).unwrap().resolve();
        action.with_valid_input(self.state, self.valid_input)
    }
}

pub struct TriggeredEvent<State, E: Event<State>> {
    state: State,
    event: E,
    input: E::Input,
}

impl<State, E: Event<State>> TriggeredEvent<State, E> {
    pub fn new(state: State, event: E, input: E::Input) -> Self {
        TriggeredEvent {
            state,
            event,
            input,
        }
    }
    pub fn consume<Listener: EventListener<State, E>>(
        self,
        handler: impl SimultaneousEventHandler<State, E>,
    ) -> EventConsume<State, E, Listener>
    where
        State: GetEventManager<State, E, Listener>,
        <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput: StateFilterInput,
    {
        let event_manager = self.state.event_manager();
        match event_manager.events.len() {
            0 => EventConsume {
                kind: EventConsumeType::Finished(self.state),
                _m: std::marker::PhantomData::default(),
            },
            1 => {
                match <Listener::Filter as StateFilter<State, E::Input>>::filter(
                    &self.state,
                    self.input,
                ) {
                    Ok(valid_output) => {
                        match <Listener::Action as ValidAction<
                            State,
                            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
                        >>::Filter::filter(&self.state, valid_output)
                        {
                            Ok(valid_output) => {
                                let mut event_actions = event_manager
                                    .collect_actions(&self.event, valid_output)
                                    .simultaneous_action_manager(self.state);
                                // UNWRAP: because length of events is 1, so there will be exactly 1 action.
                                let mut action = event_actions.actions.pop().unwrap();
                                let result = action.resolve().with_valid_input(
                                    event_actions.state,
                                    event_actions.valid_input,
                                );
                                EventConsume {
                                    kind: EventConsumeType::Result(result),
                                    _m: std::marker::PhantomData::default(),
                                }
                            }
                            Err(error) => EventConsume {
                                kind: EventConsumeType::ActionFizzle {
                                    state: self.state,
                                    error,
                                },
                                _m: std::marker::PhantomData::default(),
                            },
                        }
                    }
                    Err(error) => EventConsume {
                        kind: EventConsumeType::EventFizzle {
                            state: self.state,
                            error,
                        },
                        _m: std::marker::PhantomData::default(),
                    },
                }
            }
            _ => {
                match <Listener::Filter as StateFilter<State, E::Input>>::filter(
                    &self.state,
                    self.input,
                ) {
                    Ok(valid_output) => {
                        match <Listener::Action as ValidAction<
                            State,
                            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
                        >>::Filter::filter(&self.state, valid_output)
                        {
                            Ok(valid_output) => {
                                let event_actions = event_manager
                                    .collect_actions(&self.event, valid_output)
                                    .simultaneous_action_manager(self.state);
                                EventConsume {
                                    kind: EventConsumeType::SimultaneousActions(event_actions),
                                    _m: std::marker::PhantomData::default(),
                                }
                            }
                            Err(error) => EventConsume {
                                kind: EventConsumeType::ActionFizzle {
                                    state: self.state,
                                    error,
                                },
                                _m: std::marker::PhantomData::default(),
                            },
                        }
                    }
                    Err(error) => EventConsume {
                        kind: EventConsumeType::EventFizzle {
                            state: self.state,
                            error,
                        },
                        _m: std::marker::PhantomData::default(),
                    },
                }
            }
        }
    }
}
pub struct EventConsume<State, E: Event<State>, Listener: EventListener<State, E>> {
    kind: EventConsumeType<
        State,
        <Listener::Filter as StateFilter<State, E::Input>>::Error,
        <<Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
        >>::Filter as StateFilter<
            State,
            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
        >>::Error,
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
        >>::Output,
        SimultaneousActionManager<State, E, Listener>,
    >,
    _m: std::marker::PhantomData<(E, Listener)>,
}
pub enum EventConsumeType<State, E0, E1, R, ActionManager> {
    Finished(State),
    EventFizzle { state: State, error: E0 },
    ActionFizzle { state: State, error: E1 },
    Result(R),
    SimultaneousActions(ActionManager),
}
pub trait SimultaneousEventHandler<State, E: Event<State>> {
    fn handle_simultaneous_events(self, event: TriggeredEvent<State, E>);
}
pub trait GetEventManager<State, E: Event<State>, Listener: EventListener<State, E>> {
    fn event_manager(&self) -> &EventManager<State, E, Listener>;
}
pub trait GetEventManagerMut<State, E: Event<State>, Listener: EventListener<State, E>>:
    GetEventManager<State, E, Listener>
{
    fn event_manager_mut(&mut self) -> &mut EventManager<State, E, Listener>;
}

impl<State, E: Event<State>, Listener: EventListener<State, E>> EventManager<State, E, Listener> {}

pub trait Event<State>: Sized {
    type Input: StateFilterInput;
    //fn event_id() -> EventID;
}

pub trait EventListenerConstructor<State, E: Event<State>>: Sized {
    type Input;
    fn new_listener(source_card_id: SourceCardID, input: Self::Input) -> Self;
}
pub trait EventListener<State, E: Event<State>>: EventListenerConstructor<State, E>
where
    <Self::Filter as StateFilter<State, E::Input>>::ValidOutput: StateFilterInput,
{
    type Action: ValidAction<State, <Self::Filter as StateFilter<State, E::Input>>::ValidOutput>;
    /// Trigger event ONLY if this filter passes!
    type Filter: StateFilter<State, E::Input>;
    /// The action to execute when its event is triggered.
    fn action(&self, event: &E) -> Self::Action;
}

#[cfg(test)]
mod tests {
    use card_game_derive::StateFilterInput;

    use crate::cards::ActionID;

    use super::*;

    struct Game;
    struct Summoned;
    use crate as card_game;
    #[derive(StateFilterInput)]
    struct SummonedInput;
    impl Event<Game> for Summoned {
        type Input = SummonedInput;
    }
    #[derive(Clone)]
    struct SummonedListenerWhileInBattleZone;
    impl EventListener<Game, SummonedInput> for SummonedListenerWhileInBattleZone {
        type Filter = SummonedFilter;
        type Action = SummonedAction;
        fn action(self) -> Self::Action {
            SummonedAction
        }
    }
    #[derive(Clone)]
    struct EnteredBattleZoneListenerWhileInBattleZone;
    impl EventListener<Game, SummonedInput> for EnteredBattleZoneListenerWhileInBattleZone {
        type Filter = SummonedFilter;
        type Action = EnteredBattleZoneAction;
        fn action(self) -> Self::Action {
            EnteredBattleZoneAction
        }
    }
    #[derive(Clone)]
    enum SummonedEventListener {
        Summoned(SummonedListenerWhileInBattleZone),
        EnteredBattleZone(EnteredBattleZoneListenerWhileInBattleZone),
    }
    impl EventListener<Game, SummonedInput> for SummonedEventListener {
        type Filter = SummonedFilter;
        type Action = EnteredBattleZoneAction;
        fn action(self) -> Self::Action {
            EnteredBattleZoneAction
        }
    }
    pub enum AllSummonedActions {
        Summon(SummonedAction),
        Entered(EnteredBattleZoneAction),
    }
    impl ValidAction<Game, SummonedInput> for AllSummonedActions {
        type Filter = ();
        type Output = ();
        fn with_valid_input(
            self,
            state: Game,
            valid: <Self::Filter as StateFilter<Game, SummonedInput>>::ValidOutput,
        ) -> Self::Output {
        }
        fn action_id() -> ActionID {
            match self {
                AllSummonedActions::Summon(summoned) => summoned.action_id(),
                AllSummonedActions::Entered(entered) => entered.action_id(),
            }
        }
    }
    enum AllSummonedActionsOutput {
        Summon(<SummonedAction as ValidAction<Game, SummonedInput>>::Output),
        Entered(<EnteredBattleZoneAction as ValidAction<Game, SummonedInput>>::Output),
    }
    struct SummonedFilter;
    impl StateFilter<Game, SummonedInput> for SummonedFilter {
        type ValidOutput = SummonedInput;
        //type Error = SummonedError;
        type Error = std::convert::Infallible;
        fn filter(state: &Game, value: SummonedInput) -> Result<Self::ValidOutput, Self::Error> {
            Ok(value)
        }
    }
    #[derive(thiserror::Error, Debug)]
    #[error("test summoned error")]
    struct SummonedError;
    struct SummonedAction;
    impl ValidAction<Game, SummonedInput> for SummonedAction {
        type Filter = SummonedFilter;
        type Output = ();
        fn with_valid_input(
            self,
            _state: Game,
            _valid: <Self::Filter as StateFilter<Game, SummonedInput>>::ValidOutput,
        ) -> Self::Output {
            ()
        }
        fn action_id() -> ActionID {
            ActionID::new("summoned_action")
        }
    }
    struct EnteredBattleZoneAction;
    impl ValidAction<Game, SummonedInput> for EnteredBattleZoneAction {
        type Filter = SummonedFilter;
        type Output = ();
        fn with_valid_input(
            self,
            _state: Game,
            _valid: <Self::Filter as StateFilter<Game, SummonedInput>>::ValidOutput,
        ) -> Self::Output {
            ()
        }
        fn action_id() -> ActionID {
            ActionID::new("test_summoned_action")
        }
    }
    struct GameSimultaneousEventHandler;
    impl SimultaneousEventHandler<Game, Summoned> for GameSimultaneousEventHandler {
        fn handle_simultaneous_events(self, event: TriggeredEvent<Game, Summoned>) {
            todo!()
        }
    }
    #[test]
    fn t() {
        let mut event_manager_0 =
            EventManager::<Game, Summoned, SummonedListenerWhileInBattleZone>::new();
        //let mut event_manager_1 = EventManager::new();
        //TriggeredEvent::<Game, Summoned, SummonedListenerWhileInBattleZone>::new(state, input);
    }
}
