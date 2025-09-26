use card_game::cards::Card;

use crate::cards::{
    monster::{MonsterCard, MonsterCardType},
    spell::SpellCard,
};

pub mod monster;
pub mod specifics;
pub mod spell;
pub mod trap;

pub trait CardName {
    fn name(&self) -> &Name;
}

impl<T: CardName> CardName for Card<T> {
    fn name(&self) -> &Name {
        self.kind().name()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);
impl Name {
    pub fn new(name: String) -> Self {
        Name(name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

pub enum CardKind {
    Monster(MonsterCardType),
    Spell(SpellCard),
}
impl CardName for CardKind {
    fn name(&self) -> &Name {
        match self {
            CardKind::Monster(monster) => monster.name(),
            CardKind::Spell(spell) => spell.name(),
        }
    }
}

impl From<MonsterCardType> for CardKind {
    fn from(card: MonsterCardType) -> Self {
        CardKind::Monster(card)
    }
}
impl From<MonsterCard> for CardKind {
    fn from(card: MonsterCard) -> Self {
        CardKind::Monster(MonsterCardType::Monster(card))
    }
}

impl From<SpellCard> for CardKind {
    fn from(spell: SpellCard) -> Self {
        CardKind::Spell(spell)
    }
}
