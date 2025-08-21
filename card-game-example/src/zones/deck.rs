use card_game::{
    cards::{Card, CardID},
    zones::{ArrayZone, InfiniteZone, Zone, ZoneCardID, ZoneContext},
};
use indexmap::IndexMap;

use crate::cards::CardKind;

pub struct DeckZone {
    cards: IndexMap<CardID, Card<CardKind>>,
}

impl DeckZone {
    pub fn new() -> Self {
        DeckZone {
            cards: IndexMap::new(),
        }
    }
}

impl InfiniteZone for DeckZone {
    fn add_card(&mut self, card: Card<Self::CardKind>) {
        self.cards.insert(card.id(), card);
    }
}
impl ArrayZone for DeckZone {
    fn remove_card<'id>(&mut self, zone_card_id: ZoneCardID<'id, Self>) -> Card<Self::CardKind> {
        zone_card_id.remove(|card| self.cards.shift_remove(&card.card_id()))
    }
}
impl Zone for DeckZone {
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
