use std::collections::BTreeMap;

use card_game::{
    cards::{Card, CardID},
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
    zones::{ArrayZone, FiniteZone, InfiniteZone, Zone},
};
use indexmap::{IndexMap, map::Slice};

use crate::{cards::CardKind, filters::CardIn, zones::GetZone};

pub struct HandZone {
    player_id: PlayerID,
    cards: IndexMap<CardID, Card<CardKind>>,
}

impl HandZone {
    pub fn new(player_id: PlayerID) -> Self {
        HandZone {
            player_id,
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
    fn remove_card(&mut self, zone_card_id: ValidCardID<CardIn<Self>>) -> Card<Self::CardKind> {
        zone_card_id.remove(|id| self.cards.remove(&id.id()))
    }
}
impl Zone for HandZone {
    type CardKind = CardKind;
    type CardFilter = CardIn<Self>;
    fn player_id(&self) -> PlayerID {
        self.player_id
    }
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
impl GetZone for HandZone {
    fn get_zone<'a, F>(game: &'a crate::Game, valid_player_id: &'a ValidPlayerID<F>) -> &'a Self {
        game.zone_manager()
            .get_valid_zone(valid_player_id)
            .hand_zone()
    }
}
