use card_game::{
    identifications::{PlayerID, ValidPlayerID},
    zones::Zone,
};

use crate::{
    Game,
    filters::CardIn,
    zones::{
        deck::DeckZone, graveyard::GraveyardZone, hand::HandZone, monster::MonsterZone,
        spell::SpellZone,
    },
};

pub mod deck;
pub mod graveyard;
pub mod hand;
pub mod monster;
pub mod spell;

pub struct Zones {
    pub(crate) monster_zone: MonsterZone,
    pub(crate) spell_zone: SpellZone,
    pub(crate) graveyard_zone: GraveyardZone,
    pub(crate) deck_zone: DeckZone,
    pub(crate) hand_zone: HandZone,
}

impl Zones {
    pub fn hand_zone(&self) -> &HandZone {
        &self.hand_zone
    }
}

impl card_game::zones::Zones for Zones {
    fn new(player_id: PlayerID) -> Self {
        Zones {
            monster_zone: MonsterZone::new(player_id),
            spell_zone: SpellZone::new(player_id),
            graveyard_zone: GraveyardZone::new(player_id),
            deck_zone: DeckZone::new(player_id),
            hand_zone: HandZone::new(player_id),
        }
    }
}

pub trait GetZone: Zone<CardFilter = CardIn<Self>> {
    fn get_zone<'a, F>(game: &'a Game, player_id: &'a ValidPlayerID<F>) -> &'a Self;
}

#[derive(Debug)]
pub struct SlotID(usize);
impl SlotID {
    pub fn new(id: usize) -> Self {
        SlotID(id)
    }
}
