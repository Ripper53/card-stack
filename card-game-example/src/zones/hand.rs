use std::collections::BTreeMap;

use card_game::{
    cards::{Card, CardID},
    zones::{ArrayZone, FiniteZone, InfiniteZone, Zone, ZoneCardID},
};
use indexmap::IndexMap;

use crate::cards::CardKind;

pub struct HandZone<'a> {
    cards: IndexMap<ZoneCardID<'a, Self>, Card<CardKind>>,
}

impl<'a> HandZone<'a> {
    pub fn new() -> Self {
        HandZone {
            cards: IndexMap::new(),
        }
    }
}

impl<'a> FiniteZone<'a> for HandZone<'a> {
    fn max_count(&self) -> usize {
        10
    }
    fn add_card_unchecked(
        &mut self,
        zone_card_id: ZoneCardID<'a, Self>,
        card: Card<Self::CardKind>,
    ) {
        self.cards.insert(zone_card_id, card).unwrap();
    }
}
impl<'a> ArrayZone<'a> for HandZone<'a> {
    fn remove_card(&mut self, zone_card_id: ZoneCardID<'a, Self>) -> Card<Self::CardKind> {
        zone_card_id.remove(|id| self.cards.remove(id))
    }
}
impl<'a> Zone for HandZone<'a> {
    type CardKind = CardKind;
    fn filled_count(&self) -> usize {
        self.cards.len()
    }
}
