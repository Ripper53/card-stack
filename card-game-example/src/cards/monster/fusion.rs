use crate::cards::monster::MonsterCard;

#[derive(Debug)]
pub struct FusionMonsterCard {
    monster: MonsterCard,
}

impl FusionMonsterCard {
    pub fn new(monster: MonsterCard) -> Self {
        FusionMonsterCard { monster }
    }
}
