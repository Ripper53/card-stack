use card_game::identifications::PlayerID;

use crate::zones::{
    deck::DeckZone, graveyard::GraveyardZone, hand::HandZone, monster::MonsterZone,
    spell::SpellZone,
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
    pub fn new() -> Self {
        Zones {
            monster_zone: MonsterZone::new(),
            spell_zone: SpellZone::new(),
            graveyard_zone: GraveyardZone::new(),
            deck_zone: DeckZone::new(),
            hand_zone: HandZone::new(),
        }
    }
    pub fn hand_zone(&self) -> &HandZone {
        &self.hand_zone
    }
}

impl card_game::zones::Zones for Zones {
    fn new(player_id: PlayerID) -> Self {
        Zones::new()
    }
}
