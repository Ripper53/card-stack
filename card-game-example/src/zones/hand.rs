use std::collections::BTreeMap;

use card_game::{
    cards::{Card, CardID},
    zones::Zone,
};

use crate::cards::CardKind;

pub struct HandZone {
    cards: BTreeMap<CardID, Card<CardKind>>,
}

impl HandZone {
    pub fn new() -> Self {
        HandZone {
            cards: BTreeMap::new(),
        }
    }
}

impl Zone for HandZone {
    type CardKind = CardKind;
    fn filled_count(&self) -> usize {
        self.cards.len()
    }
}
