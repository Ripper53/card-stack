use crate::events::{
    AddEventListener, DynEventListener, Event, EventListener, EventListenerConstructor,
};
use crate::identifications::{ActionIdentifier, SourceCardID, ValidCardID};
use card_stack::priority::PriorityMut;
use card_stack::{NonEmptyInput, priority::GetState};
use state_validation::{
    StateFilter, StateFilterInputCombination, StateFilterInputConversion, ValidAction,
};

mod manager;
pub use manager::*;

#[derive(Debug, Clone)]
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
    pub fn replace_kind<NewKind>(self, f: impl FnOnce(Kind) -> NewKind) -> Card<NewKind> {
        Card {
            id: self.id,
            kind: f(self.kind),
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardID(usize);
impl NonEmptyInput for CardID {}
impl CardID {
    pub const fn new(id: usize) -> Self {
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
    card_event_tracker: &'a mut CardEventTracker<EventManager>,
    next_id: &'a mut usize,
}

impl<'a, EventManager> CardBuilder<'a, EventManager> {
    pub(crate) fn new(
        card_actions: &'a mut CardActions,
        event_manager: &'a mut EventManager,
        card_event_tracker: &'a mut CardEventTracker<EventManager>,
        next_id: &'a mut usize,
    ) -> Self {
        CardBuilder {
            card_actions,
            event_manager,
            card_event_tracker,
            next_id,
        }
    }
    pub fn build<Kind>(&mut self, kind: Kind) -> CardKindBuilder<'_, EventManager, Kind> {
        let id = CardID::new(*self.next_id);
        *self.next_id += 1;
        CardKindBuilder {
            card_actions: self.card_actions,
            event_manager: self.event_manager,
            card_event_tracker: self.card_event_tracker,
            card: Card::new(id, kind),
        }
    }
}

pub struct CardKindBuilder<'a, EventManager, Kind> {
    card_actions: &'a mut CardActions,
    event_manager: &'a mut EventManager,
    card_event_tracker: &'a mut CardEventTracker<EventManager>,
    card: Card<Kind>,
}

impl<'a, EventManager, Kind> CardKindBuilder<'a, EventManager, Kind> {
    pub fn with_action<Action: ActionIdentifier>(self) -> Self {
        self.card_actions
            .insert_action(Action::action_id(), self.card.id());
        self
    }
    pub fn copy_actions(self, card_id: CardID) -> Self {
        todo!()
    }
    pub fn with_event<
        State: 'static,
        Ev: Event<PriorityMut<State>>,
        Listener: EventListenerConstructor<State, Ev>,
    >(
        self,
        listener_input: Listener::Input,
    ) -> Self
    where
        Listener::Input: Clone,
        <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
            State,
            Listener,
        >>::ValidOutput: 'static,
        <<Listener as EventListener<State, Ev>>::Filter as StateFilter<
            State,
            Listener,
        >>::Error: 'static,
        <<<Listener as EventListener<State, Ev>>::Action as ValidAction<
            PriorityMut<State>,
            Listener::ActionInput,
        >>::Filter as StateFilter<
            PriorityMut<State>,
            Listener::ActionInput,
        >>::ValidOutput: 'static,
        <<<Listener as EventListener<State, Ev>>::Action as ValidAction<
            PriorityMut<State>,
            Listener::ActionInput,
        >>::Filter as StateFilter<
            PriorityMut<State>,
            Listener::ActionInput,
        >>::Error: 'static,
        EventManager: AddEventListener<State, Ev>,
        EventManager::Output: 'static,
        <Listener::Action as ValidAction<
            PriorityMut< State>,
            Listener::ActionInput,
        >>::Output: Into<EventManager::Output>,
    {
        let card_id = self.card.id();
        let event_listener = Listener::new_listener(SourceCardID(card_id), listener_input.clone());
        self.card_event_tracker
            .track_event::<_, _, Listener>(card_id, listener_input);
        self.event_manager.add_listener(event_listener);
        self
    }
    pub fn copy_events(self, card_id: CardID) -> Self {
        self.card_event_tracker
            .copy_events(self.event_manager, self.card.id(), card_id);
        self
    }
    pub fn finish(self) -> Card<Kind> {
        self.card
    }
}

pub struct SourceCardFilter<Action>(std::marker::PhantomData<Action>);
impl<
    Input: StateFilterInputConversion<SourceCardID>,
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
