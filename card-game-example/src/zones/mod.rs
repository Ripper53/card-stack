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

pub struct Zones<'a> {
    pub(crate) monster_zone: MonsterZone<'a>,
    pub(crate) spell_zone: SpellZone<'a>,
    pub(crate) graveyard_zone: GraveyardZone,
    pub(crate) deck_zone: DeckZone<'a>,
    pub(crate) hand_zone: HandZone<'a>,
}

impl<'a> Zones<'a> {
    pub fn new() -> Self {
        Zones {
            monster_zone: MonsterZone::new(),
            spell_zone: SpellZone::new(),
            graveyard_zone: GraveyardZone::new(),
            deck_zone: DeckZone::new(),
            hand_zone: HandZone::new(),
        }
    }
}

impl<'a> card_game::zones::Zones for Zones<'a> {
    fn new(player_id: PlayerID) -> Self {
        Zones::new()
    }
}
