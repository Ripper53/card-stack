mod manager;
use card_stack::priority::GetState;
pub use manager::*;

use crate::events::{
    DynEventListener, Event, EventListener, EventListenerConstructor, GetEventManagerMut,
};
use crate::identifications::{ActionIdentifier, SourceCardID, ValidCardID};
use state_validation::{
    StateFilter, StateFilterInput, StateFilterInputCombination, StateFilterInputConversion,
    ValidAction,
};

pub struct Card<Kind> {
    id: CardID,
    kind: Kind,
}

impl<Kind> Card<Kind> {
    pub fn new(card_id: CardID, kind: Kind) -> Self {
        Card { id: card_id, kind }
    }
    pub fn id(&self) -> CardID {
        self.id
    }
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
    pub fn kind_mut(&mut self) -> &mut Kind {
        &mut self.kind
    }
    pub fn take_kind(self) -> Kind {
        self.kind
    }
    pub fn into_kind<NewKind>(self) -> Card<NewKind>
    where
        Kind: Into<NewKind>,
    {
        Card {
            id: self.id,
            kind: self.kind.into(),
        }
    }
}

#[derive(StateFilterInput, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardID(usize);
impl CardID {
    pub(crate) const fn new(id: usize) -> Self {
        CardID(id)
    }
    pub(crate) fn clone_id(&self) -> Self {
        CardID::new(self.0)
    }
}
impl std::fmt::Display for CardID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub struct CardBuilder<'a, EventManager> {
    card_actions: &'a mut CardActions,
    event_manager: &'a mut EventManager,
    next_id: &'a mut usize,
}

impl<'a, EventManager> CardBuilder<'a, EventManager> {
    pub(crate) fn new(
        card_actions: &'a mut CardActions,
        event_manager: &'a mut EventManager,
        next_id: &'a mut usize,
    ) -> Self {
        CardBuilder {
            card_actions,
            event_manager,
            next_id,
        }
    }
    pub fn build<Kind>(&mut self, kind: Kind) -> CardKindBuilder<'_, EventManager, Kind> {
        let id = CardID::new(*self.next_id);
        *self.next_id += 1;
        CardKindBuilder {
            card_actions: self.card_actions,
            event_manager: self.event_manager,
            card: Card::new(id, kind),
        }
    }
}

pub struct CardKindBuilder<'a, EventManager, Kind> {
    card_actions: &'a mut CardActions,
    event_manager: &'a mut EventManager,
    card: Card<Kind>,
}

impl<'a, EventManager, Kind> CardKindBuilder<'a, EventManager, Kind> {
    pub fn with_action<Action: ActionIdentifier>(self) -> Self {
        self.card_actions
            .insert_action(Action::action_id(), self.card.id());
        self
    }
    pub fn with_event<
        State: 'static,
        Ev: Event<State>,
        Listener: EventListener<State, Ev> + EventListenerConstructor<State, Ev>,
    >(
        self,
        listener_input: Listener::Input,
    ) -> Self
    where
        <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
            State,
            <Ev as Event<State>>::Input,
        >>::ValidOutput: 'static,
        <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
            State,
            <Ev as Event<State>>::Input,
        >>::Error: 'static,
        <<<Listener as EventListener<State, Ev>>::Action as ValidAction<
            State,
            <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
                State,
                <Ev as Event<State>>::Input,
            >>::ValidOutput,
        >>::Filter as StateFilter<
            State,
            <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
                State,
                <Ev as Event<State>>::Input,
            >>::ValidOutput,
        >>::ValidOutput: 'static,
        <<<Listener as EventListener<State, Ev>>::Action as ValidAction<
            State,
            <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
                State,
                <Ev as Event<State>>::Input,
            >>::ValidOutput,
        >>::Filter as StateFilter<
            State,
            <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
                State,
                <Ev as Event<State>>::Input,
            >>::ValidOutput,
        >>::Error: 'static,
        EventManager: GetEventManagerMut<State, Ev>,
        EventManager::Output: 'static,
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Output: Into<EventManager::Output>,
    {
        self.event_manager
            .event_manager_mut()
            .add_listener(Listener::new_listener(
                SourceCardID(self.card.id()),
                listener_input,
            ));
        self
    }
    /*pub fn with_event<
        State,
        Ev: Event<State>,
        Listener: EventListener<State, Ev> + EventListenerConstructor<State, Ev>,
    >(
        self,
        input: Listener::Input,
    ) -> Self
    where
        EventManager: GetEventManagerMut<State, Ev, Listener>,
    {
        self.event_manager
            .event_manager_mut()
            .add_listener(Listener::new_listener(SourceCardID(self.card.id()), input));
        self
    }*/
    pub fn finish(self) -> Card<Kind> {
        self.card
    }
}

pub struct SourceCardFilter<Action>(std::marker::PhantomData<Action>);
impl<
    Input: StateFilterInput + StateFilterInputConversion<SourceCardID>,
    Action: ValidAction<State, Input> + ActionIdentifier,
    State: GetState<CardActions>,
> StateFilter<State, Input> for SourceCardFilter<Action>
where
    Input::Remainder: StateFilterInputCombination<ValidCardID<()>>,
{
    type ValidOutput = <Input::Remainder as StateFilterInputCombination<ValidCardID<()>>>::Combined;
    type Error = InvalidSourceCardError;
    fn filter(state: &State, value: Input) -> Result<Self::ValidOutput, Self::Error> {
        let (source_card_id, remainder) = value.split_take();
        if state
            .state()
            .contains_action(Action::action_id(), source_card_id.0)
        {
            Ok(remainder.combine(ValidCardID::new(source_card_id.0)))
        } else {
            Err(InvalidSourceCardError(source_card_id))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("invalid source card {0}")]
pub struct InvalidSourceCardError(SourceCardID);
