use card_game::{
    cards::CardID,
    identifications::{ActivePlayer, PlayerID, ValidCardID, ValidPlayerID},
    validation::{Condition, StateFilter, ValidAction},
    zones::{ArrayZone, Zone},
};

use crate::{
    Game,
    cards::monster::{MonsterCard, Position},
    filters::{CardIn, FilterInput, For, Free, In, MonsterSlot, OfType, With},
    steps::MainStep,
    zones::{SlotID, hand::HandZone, monster::MonsterZone},
};

pub struct PlayMonsterCardValidAction {
    position: Position,
}
impl PlayMonsterCardValidAction {
    pub fn new(position: Position) -> Self {
        PlayMonsterCardValidAction { position }
    }
}

impl ValidAction<MainStep, FilterInput<(PlayerID, CardID, SlotID)>> for PlayMonsterCardValidAction {
    type Filter = (
        Condition<FilterInput<(PlayerID, CardID)>, CardIn<HandZone>>,
        Condition<
            FilterInput<(ValidPlayerID<()>, ValidCardID<CardIn<HandZone>>)>,
            OfType<MonsterCard>,
        >,
        //Condition<(ValidPlayerID<()>, SlotID), With<(Free<MonsterSlot>, In<MonsterZone>)>>,
        //For<ActivePlayer>,
    );
    type Output = MainStep;
    fn with_valid_input(
        self,
        mut state: MainStep,
        _: <Self::Filter as StateFilter<MainStep, FilterInput<(PlayerID, CardID, SlotID)>>>::ValidOutput,
        /*(valid_player_id, valid_card_id, slot_index): <Self::Filter as StateFilter<
            MainStep,
        >>::ValidOutput,*/
    ) -> Self::Output {
        //if state.game.active_player_zones_mut().monster_zone.get_card_from_index(self.slot_index)
        /*let card = state
        .game
        .zone_manager_mut()
        .valid_zone_mut(valid_player_id)
        .hand_zone_mut()
        .remove_monster_card(valid_card_id.into());*/
        //state.game
        //.active_player_zones_mut().monster_zone.
        state
    }
}
