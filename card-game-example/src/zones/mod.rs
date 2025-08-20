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
    monster_zone: MonsterZone<'a>,
    spell_zone: SpellZone<'a>,
    graveyard_zone: GraveyardZone,
    deck_zone: DeckZone<'a>,
    hand_zone: HandZone,
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
