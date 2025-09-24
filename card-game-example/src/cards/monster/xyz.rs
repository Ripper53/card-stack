use crate::cards::monster::MonsterCard;

pub struct XyzMonsterCard {
    monster: MonsterCard,
}

impl XyzMonsterCard {
    pub fn new(monster: MonsterCard) -> Self {
        XyzMonsterCard { monster }
    }
}
