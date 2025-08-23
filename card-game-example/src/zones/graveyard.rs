use std::collections::BTreeMap;

use card_game::{
    cards::{Card, CardID},
    identifications::PlayerID,
};

use crate::cards::CardKind;

pub struct GraveyardZone {
    player_id: PlayerID,
    cards: BTreeMap<CardID, Card<CardKind>>,
}

impl GraveyardZone {
    pub fn new(player_id: PlayerID) -> Self {
        GraveyardZone {
            player_id,
            cards: BTreeMap::new(),
        }
    }
}
