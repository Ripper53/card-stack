use crate::cards::monster::MonsterCard;

pub struct FusionMonsterCard {
    monster: MonsterCard,
}

impl FusionMonsterCard {
    pub fn new(monster: MonsterCard) -> Self {
        FusionMonsterCard { monster }
    }
}
