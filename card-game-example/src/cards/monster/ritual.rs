use crate::cards::monster::MonsterCard;

pub struct RitualMonsterCard {
    monster: MonsterCard,
}

impl RitualMonsterCard {
    pub fn new(monster: MonsterCard) -> Self {
        RitualMonsterCard { monster }
    }
}
