use card_game::{
    StateFilterInput,
    cards::{ActionID, CardID},
    identifications::{ActivePlayer, PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{Condition, StateFilter, ValidAction},
};

use crate::{
    Game,
    cards::monster::Position,
    filters::{CardIn, FilterInput, For, Free, In, MonsterSlot, With},
    steps::MainStep,
    zones::{SlotID, hand::HandZone, monster::MonsterZone},
};

pub struct TributeSummon {
    position: Position,
}

impl TributeSummon {
    pub fn new(position: Position) -> Self {
        TributeSummon { position }
    }
}

#[derive(StateFilterInput)]
pub struct Tribute(pub CardID);
#[derive(StateFilterInput)]
pub struct ValidTribute(pub ValidCardID<CardIn<MonsterZone>>);

impl ValidAction<MainStep, FilterInput<(PlayerID, CardID, SlotID, Tribute)>> for TributeSummon {
    type Filter = (
        Condition<FilterInput<PlayerID>, For<ActivePlayer>>,
        Condition<FilterInput<(ValidPlayerID<ActivePlayer>, CardID)>, CardIn<HandZone>>,
        Condition<
            FilterInput<(ValidPlayerID<ActivePlayer>, SlotID)>,
            With<(Free<MonsterSlot>, In<MonsterZone>)>,
        >,
        Condition<FilterInput<(ValidPlayerID<ActivePlayer>, Tribute)>, CardIn<MonsterZone>>,
    );
    type Output = (ValidPlayerID<ActivePlayer>, ValidTribute);
    fn with_valid_input(
        self,
        state: MainStep,
        valid: <Self::Filter as StateFilter<
            MainStep,
            FilterInput<(PlayerID, CardID, SlotID, Tribute)>,
        >>::ValidOutput,
    ) -> Self::Output {
        todo!()
    }
    fn action_id() -> ActionID {
        ActionID::new("tribute_summon")
    }
}
