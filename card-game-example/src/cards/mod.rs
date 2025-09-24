use card_game::cards::Card;

use crate::cards::{monster::MonsterCard, spell::SpellCard};

pub mod monster;
pub mod specifics;
pub mod spell;
pub mod trap;

#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);
impl Name {
    pub fn new(name: String) -> Self {
        Name(name)
    }
}

pub enum CardKind {
    Monster(MonsterCard),
    Spell(SpellCard),
}

impl From<MonsterCard> for CardKind {
    fn from(card: MonsterCard) -> Self {
        CardKind::Monster(card)
    }
}

impl From<SpellCard> for CardKind {
    fn from(spell: SpellCard) -> Self {
        CardKind::Spell(spell)
    }
}
