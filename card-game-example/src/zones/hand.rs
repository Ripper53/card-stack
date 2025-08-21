use std::collections::BTreeMap;

use card_game::{
    cards::{Card, CardID},
    zones::{ArrayZone, FiniteZone, InfiniteZone, Zone, ZoneCardID},
};
use indexmap::{IndexMap, map::Slice};

use crate::cards::CardKind;

pub struct HandZone {
    cards: IndexMap<CardID, Card<CardKind>>,
}

impl HandZone {
    pub fn new() -> Self {
        HandZone {
            cards: IndexMap::new(),
        }
    }
}

impl FiniteZone for HandZone {
    fn max_count(&self) -> usize {
        10
    }
    fn add_card_unchecked(&mut self, card: Card<Self::CardKind>) {
        self.cards.insert(card.id(), card).unwrap();
    }
}
impl ArrayZone for HandZone {
    fn remove_card<'id>(&mut self, zone_card_id: ZoneCardID<'id, Self>) -> Card<Self::CardKind> {
        zone_card_id.remove(|id| self.cards.remove(&id.card_id()))
    }
}
impl Zone for HandZone {
    type CardKind = CardKind;
    fn filled_count(&self) -> usize {
        self.cards.len()
    }
    fn get_card(&self, card_id: CardID) -> Option<&Card<Self::CardKind>> {
        self.cards.get(&card_id)
    }
    fn get_card_from_index(&self, index: usize) -> Option<&Card<Self::CardKind>> {
        self.cards.get_index(index).map(|(_k, v)| v)
    }
    fn cards(&self) -> impl Iterator<Item = &Card<Self::CardKind>> {
        self.cards.iter().map(|(card_id, card)| card)
    }
}
