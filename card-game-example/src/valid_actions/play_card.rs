use card_game::{
    cards::{Card, CardID},
    identifications::{ActivePlayer, PlayerID, ValidCardID, ValidPlayerID},
    validation::{Condition, StateFilter, ValidAction},
    zones::{ArrayZone, Zone},
};

use crate::{
    Game,
    cards::monster::{MonsterCard, MonsterZoneCard, Position},
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
        Condition<FilterInput<PlayerID>, For<ActivePlayer>>,
        Condition<FilterInput<(ValidPlayerID<ActivePlayer>, CardID)>, CardIn<HandZone>>,
        Condition<
            FilterInput<(ValidPlayerID<ActivePlayer>, ValidCardID<CardIn<HandZone>>)>,
            OfType<MonsterCard>,
        >,
        Condition<
            FilterInput<(ValidPlayerID<ActivePlayer>, SlotID)>,
            With<(Free<MonsterSlot>, In<MonsterZone>)>,
        >,
    );
    type Output = MainStep;
    fn with_valid_input(
        self,
        mut state: MainStep,
        FilterInput((valid_player_id, valid_card_id, valid_slot_id)): <Self::Filter as StateFilter<
            MainStep,
            FilterInput<(PlayerID, CardID, SlotID)>,
        >>::ValidOutput,
    ) -> Self::Output {
        let zones = state
            .game
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id);
        let card = zones.hand_zone_mut().remove_monster_card(valid_card_id);
        let card_id = card.id();
        let card = MonsterZoneCard::new(card.take_kind().into(), self.position);
        let _ = zones
            .monster_zone
            .valid_slot(valid_slot_id)
            .put(Card::new(card_id, card).into_kind());
        state
    }
}
