mod blue_eyes_white_destiny;
pub use blue_eyes_white_destiny::*;
use card_game::cards::{Card, CardBuilder};

use crate::{
    cards::{
        Name,
        monster::{Attack, Defense, Level, MonsterCard},
    },
    events::summon::SpecialSummoned,
};

pub trait TestCards {
    fn passive_card_test(&mut self) -> Card<MonsterCard>;
}
/*impl<'a> TestCards for CardBuilder<'a> {
    fn passive_card_test(&mut self) -> Card<MonsterCard> {
        self.build(MonsterCard::new(
            Name::new("Passive Test Monster".into()),
            Level::new(4),
            Attack::new(1000),
            Defense::new(2000),
        ))
        .with_event::<_, SpecialSummoned, _>()
    }
}
*/
