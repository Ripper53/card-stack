use card_game::cards::Card;

use crate::cards::{monster::MonsterCard, spell::SpellCard};

pub mod monster;
pub mod spell;

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
