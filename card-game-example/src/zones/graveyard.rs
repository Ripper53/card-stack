use std::collections::BTreeMap;

use card_game::cards::{Card, CardID};

use crate::cards::CardKind;

pub struct GraveyardZone {
    cards: BTreeMap<CardID, Card<CardKind>>,
}

impl GraveyardZone {
    pub fn new() -> Self {
        GraveyardZone {
            cards: BTreeMap::new(),
        }
    }
}
