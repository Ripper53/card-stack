use card_game::{
    validation::{StateFilter, ValidAction, filters::CardIn},
    zones::ArrayZone,
};

use crate::{steps::MainStep, zones::hand::HandZone};

pub struct PlayCardValidAction;

impl ValidAction for PlayCardValidAction {
    type State = MainStep;
    type Filter = CardIn<HandZone>;
    type Output = MainStep;
    fn with_valid_input(
        self,
        mut state: Self::State,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter>::Valid<'_>,
    ) -> Self::Output {
        let _ = state
            .game
            .active_player_zones_mut()
            .hand_zone
            .remove_card(valid_card_id);
        //state.game
        //.active_player_zones_mut().monster_zone.
        state
    }
}
