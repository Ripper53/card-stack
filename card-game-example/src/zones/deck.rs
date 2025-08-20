use card_game::{
    cards::{Card, CardID},
    zones::{ArrayZone, InfiniteZone, Zone, ZoneCardID},
};
use indexmap::IndexMap;

use crate::cards::CardKind;

pub struct DeckZone<'a> {
    cards: IndexMap<ZoneCardID<'a, Self>, Card<CardKind>>,
}

impl<'a> DeckZone<'a> {
    pub fn new() -> Self {
        DeckZone {
            cards: IndexMap::new(),
        }
    }
}

impl<'a> InfiniteZone<'a> for DeckZone<'a> {
    fn add_card_with_id(&mut self, zone_card_id: ZoneCardID<'a, Self>, card: Card<Self::CardKind>) {
        self.cards.insert(zone_card_id, card);
    }
}
impl<'a> ArrayZone<'a> for DeckZone<'a> {
    fn remove_card(&mut self, zone_card_id: ZoneCardID<'a, Self>) -> Card<Self::CardKind> {
        zone_card_id.remove(|card| self.cards.shift_remove(card))
    }
}
impl<'a> Zone for DeckZone<'a> {
    type CardKind = CardKind;
    fn filled_count(&self) -> usize {
        self.cards.len()
    }
}
