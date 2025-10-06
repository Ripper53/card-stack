mod fusion;
mod link;
mod ritual;
mod synchro;
mod xyz;
use std::ops::AddAssign;

pub use fusion::*;
pub use link::*;
pub use ritual::*;
pub use synchro::*;
pub use xyz::*;

use crate::cards::{CardName, Name};

pub enum MonsterCardType {
    Monster(MonsterCard),
    Special(SpecialMonsterCardType),
}

impl MonsterCardType {
    pub fn add_attack(&mut self, attack: Attack) {
        match self {
            MonsterCardType::Monster(monster) => {
                monster.attack += attack;
            }
            MonsterCardType::Special(special) => match special {
                _ => todo!(),
            },
        }
    }
}

impl CardName for MonsterCardType {
    fn name(&self) -> &Name {
        match self {
            MonsterCardType::Monster(monster) => monster.name(),
            MonsterCardType::Special(special_monster) => special_monster.name(),
        }
    }
}

impl From<MonsterCard> for MonsterCardType {
    fn from(monster_card: MonsterCard) -> Self {
        MonsterCardType::Monster(monster_card)
    }
}

pub enum SpecialMonsterCardType {
    Fusion(FusionMonsterCard),
    Ritual(RitualMonsterCard),
    Synchro(SynchroMonsterCard),
    Xyz(XyzMonsterCard),
    Link(LinkMonsterCard),
}
impl CardName for SpecialMonsterCardType {
    fn name(&self) -> &Name {
        todo!()
    }
}

pub struct MonsterCard {
    name: Name,
    level: Level,
    attack: Attack,
    defense: Defense,
}
impl CardName for MonsterCard {
    fn name(&self) -> &Name {
        &self.name
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Level(usize);
impl Level {
    pub fn new(level: usize) -> Self {
        Level(level)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Attack(usize);
impl Attack {
    pub fn new(power: usize) -> Attack {
        Attack(power)
    }
}
impl AddAssign for Attack {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Defense(usize);
impl Defense {
    pub fn new(defense: usize) -> Defense {
        Defense(defense)
    }
}

impl MonsterCard {
    pub fn new(name: Name, level: Level, power: Attack, defense: Defense) -> Self {
        MonsterCard {
            name,
            level,
            attack: power,
            defense,
        }
    }
    pub fn level(&self) -> Level {
        self.level
    }
}

pub struct MonsterZoneCard {
    monster_card: MonsterCardType,
    position: Position,
}

impl CardName for MonsterZoneCard {
    fn name(&self) -> &Name {
        self.monster_card.name()
    }
}

impl MonsterZoneCard {
    pub fn kind(&self) -> &MonsterCardType {
        &self.monster_card
    }
    pub fn kind_mut(&mut self) -> &mut MonsterCardType {
        &mut self.monster_card
    }
}

pub struct SpecialMonsterZoneCard {
    monster_card: SpecialMonsterCardType,
    position: Position,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Position {
    Attack,
    Defense,
}

impl MonsterZoneCard {
    pub fn new(monster_card: MonsterCardType, position: Position) -> Self {
        MonsterZoneCard {
            monster_card,
            position,
        }
    }
}
