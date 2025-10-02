mod manager;
use card_game_derive::{StateFilterInput, impl_state_filter_inputs};
use card_stack::priority::GetState;
pub use manager::*;

use crate::events::{Event, EventListener};
use crate::identifications::{SourceCardID, ValidCardID};
use crate::validation::{
    Condition, StateFilter, StateFilterCombination, StateFilterInput, StateFilterInputConversion,
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

use crate as card_game;
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

pub struct CardBuilder<'a> {
    card_actions: &'a mut CardActions,
    next_id: &'a mut usize,
}

impl<'a> CardBuilder<'a> {
    pub(crate) fn new(card_actions: &'a mut CardActions, next_id: &'a mut usize) -> Self {
        CardBuilder {
            card_actions,
            next_id,
        }
    }
    pub fn build<Kind>(&mut self, kind: Kind) -> CardKindBuilder<'_, Kind> {
        let id = CardID::new(*self.next_id);
        *self.next_id += 1;
        CardKindBuilder {
            card_actions: self.card_actions,
            card: Card::new(id, kind),
        }
    }
}

pub struct CardKindBuilder<'a, Kind> {
    card_actions: &'a mut CardActions,
    card: Card<Kind>,
}

impl<'a, Kind> CardKindBuilder<'a, Kind> {
    pub fn with_action<
        State,
        Input: StateFilterInput + StateFilterInputConversion<SourceCardID>,
        Action: ValidAction<State, Input>,
    >(
        self,
    ) -> Self {
        self.card_actions
            .insert_action(Action::action_id(), self.card.id());
        self
    }
    pub fn with_event<State, E: Event<State>, Listener: EventListener<State, E>>(self) -> Self
    where
        E::Input: StateFilterInputConversion<SourceCardID>,
    {
        /*self.card_actions.insert_event(
            <Listener as EventListener<State, E>>::Action::action_id(),
            self.card.id(),
        );*/
        todo!();
        self
    }
    pub fn finish(self) -> Card<Kind> {
        self.card
    }
}

pub struct SourceCardFilter<Action>(std::marker::PhantomData<Action>);
impl<
    Input: StateFilterInput + StateFilterInputConversion<SourceCardID>,
    Action: ValidAction<State, Input>,
    State: GetState<CardManager>,
> StateFilter<State, Input> for SourceCardFilter<Action>
where
    Input::Remainder: StateFilterCombination<ValidCardID<()>>,
{
    type ValidOutput = <Input::Remainder as StateFilterCombination<ValidCardID<()>>>::Combined;
    type Error = InvalidSourceCardError;
    fn filter(state: &State, value: Input) -> Result<Self::ValidOutput, Self::Error> {
        let (source_card_id, remainder) = value.split_take();
        if state
            .state()
            .card_actions()
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
