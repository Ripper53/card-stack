use card_game::cards::Card;

use crate::cards::{monster::MonsterCard, spell::SpellCard};

pub mod monster;
pub mod spell;

pub enum CardKind {
    Monster(Card<MonsterCard>),
    Spell(Card<SpellCard>),
}
