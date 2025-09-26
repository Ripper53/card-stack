mod manager;
use card_game_derive::{StateFilterInput, impl_state_filter_inputs};
pub use manager::*;

use crate::validation::StateFilterInput;

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

pub struct CardBuilder {
    next_id: usize,
}

impl CardBuilder {
    pub(crate) fn new() -> Self {
        CardBuilder { next_id: 0 }
    }
    pub fn build<Kind>(&mut self, kind: Kind) -> Card<Kind> {
        let id = CardID::new(self.next_id);
        self.next_id += 1;
        Card::new(id, kind)
    }
}
