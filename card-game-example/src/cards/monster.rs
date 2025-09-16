pub struct MonsterCard {}

impl MonsterCard {
    pub fn new() -> Self {
        MonsterCard {}
    }
}

pub struct MonsterZoneCard {
    monster_card: MonsterCard,
    position: Position,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Position {
    Attack,
    Defense,
}

impl MonsterZoneCard {
    pub fn new(monster_card: MonsterCard, position: Position) -> Self {
        MonsterZoneCard {
            monster_card,
            position,
        }
    }
}
