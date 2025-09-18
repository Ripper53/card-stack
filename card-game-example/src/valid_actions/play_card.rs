use card_game::{
    cards::CardID,
    identifications::{ActivePlayer, PlayerID},
    validation::{StateFilter, ValidAction},
    zones::{ArrayZone, Zone},
};

use crate::{
    Game,
    cards::monster::{MonsterCard, Position},
    filters::{CardIn, For, OfType},
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

impl ValidAction<MainStep, (PlayerID, CardID)> for PlayMonsterCardValidAction {
    type Filter = (CardIn<HandZone>, OfType<MonsterCard>, For<ActivePlayer>);
    type Output = MainStep;
    fn with_valid_input(
        self,
        mut state: MainStep,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter<
            MainStep,
            (PlayerID, CardID),
        >>::ValidOutput,
    ) -> Self::Output {
        //if state.game.active_player_zones_mut().monster_zone.get_card_from_index(self.slot_index)
        let _ = state
            .game
            .zone_manager_mut()
            .get_valid_zone_mut(valid_player_id)
            .hand_zone
            .remove_card(valid_card_id.into());
        //state.game
        //.active_player_zones_mut().monster_zone.
        state
    }
}
