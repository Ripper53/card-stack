use card_game::{
    cards::{Card, CardID},
    events::TriggeredEvent,
    identifications::{
        ActionID, ActionIdentifier, ActivePlayer, PlayerID, ValidCardID, ValidPlayerID,
    },
    zones::{ArrayZone, Zone},
};
use state_validation::{Condition, StateFilter, ValidAction};

use crate::{
    Game,
    cards::{
        CardName,
        monster::{MonsterCard, MonsterZoneCard, Position},
    },
    events::summon::{NormalSummoned, Summoned},
    filters::{CardIn, EqualOrLowerThan, For, Free, In, Level, MonsterSlot, OfType, With},
    steps::MainStep,
    zones::{ContainsMonsterCards, SlotID, hand::HandZone, monster::MonsterZone},
};

mod input;
pub use input::*;

pub struct NormalSummon {
    position: Position,
}
impl NormalSummon {
    pub fn new(position: Position) -> Self {
        NormalSummon { position }
    }
}

impl ActionIdentifier for NormalSummon {
    fn action_id() -> ActionID {
        ActionID::new("normal_summon")
    }
}
impl ValidAction<MainStep, NormalSummonInput> for NormalSummon {
    type Filter = (
        Condition<PlayerID, For<ActivePlayer>>,
        Condition<(ValidPlayerID<ActivePlayer>, CardID), CardIn<HandZone>>,
        Condition<
            (ValidPlayerID<ActivePlayer>, ValidCardID<CardIn<HandZone>>),
            OfType<MonsterCard>,
        >,
        Condition<
            (
                ValidPlayerID<ActivePlayer>,
                ValidCardID<(CardIn<HandZone>, OfType<MonsterCard>)>,
            ),
            With<EqualOrLowerThan<Level<4>>>,
        >,
        Condition<
            (ValidPlayerID<ActivePlayer>, SlotID),
            With<(Free<MonsterSlot>, In<MonsterZone>)>,
        >,
    );
    type Output = TriggeredEvent<MainStep, NormalSummoned>;
    fn with_valid_input(
        self,
        mut state: MainStep,
        value: <Self::Filter as StateFilter<MainStep, NormalSummonInput>>::ValidOutput,
    ) -> Self::Output {
        if !state.use_normal_summon() {
            todo!("create a filter that checks if a normal summon can occur");
        }
        let player_id = value.player_id.id();
        let zones = state
            .game
            .zone_manager_mut()
            .valid_zone_mut(value.player_id);
        let card = zones
            .hand_zone_mut()
            .remove_monster_card(value.card_id.into_filter());
        let card_id = card.id();
        let card = MonsterZoneCard::new(card.take_kind().into(), self.position);
        println!("card: {}", card.name());
        let _ = zones
            .monster_zone
            .valid_slot(value.slot_id)
            .put(Card::new(card_id, card).into_kind());
        TriggeredEvent::new(
            state,
            NormalSummoned { player_id, card_id },
            NormalSummoned { player_id, card_id },
        )
    }
}
