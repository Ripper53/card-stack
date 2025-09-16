use card_game::{
    validation::{StateFilter, ValidAction, filters::CardIn},
    zones::{ArrayZone, Zone},
};

use crate::{
    cards::monster::{MonsterCard, Position},
    filters::OfType,
    steps::MainStep,
    zones::hand::HandZone,
};

pub struct PlayMonsterCardValidAction {
    slot_index: usize,
    position: Position,
}
impl PlayMonsterCardValidAction {
    pub fn new(slot_index: usize, position: Position) -> Self {
        PlayMonsterCardValidAction {
            slot_index,
            position,
        }
    }
}

impl ValidAction for PlayMonsterCardValidAction {
    type State = MainStep;
    type Filter = CardIn<(HandZone, OfType<MonsterCard>)>;
    type Output = MainStep;
    fn with_valid_input(
        self,
        mut state: Self::State,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter>::Valid<'_>,
    ) -> Self::Output {
        //if state.game.active_player_zones_mut().monster_zone.get_card_from_index(self.slot_index)
        let _ = state
            .game
            .active_player_zones_mut()
            .hand_zone
            .remove_card(valid_card_id.into());
        //state.game
        //.active_player_zones_mut().monster_zone.
        state
    }
}
