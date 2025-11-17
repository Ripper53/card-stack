use card_game::{
    cards::CardID,
    create_valid_identification,
    identifications::{
        ActionID, ActionIdentifier, ActivePlayer, PlayerID, ValidCardID, ValidPlayerID,
    },
    stack::{NonEmptyInput, priority::GetState},
};
use state_validation::{Condition, StateFilter, StateFilterInput, ValidAction};

use crate::{
    Game,
    cards::monster::Position,
    filters::{CardIn, For, Free, In, MonsterSlot, With},
    identifications::ValidSlotID,
    steps::MainStep,
    zones::{SlotID, hand::HandZone, monster::MonsterZone},
};

#[derive(StateFilterInput)]
pub struct TributeSummonInput {
    #[conversion(T0 = ValidPlayerID<T0>)]
    pub player_id: PlayerID,
    #[conversion(T1 = ValidCardID<T1>)]
    pub card_id: CardID,
    #[conversion(T2 = ValidSlotID<T2>)]
    pub slot_id: SlotID,
    #[conversion(T3 = ValidTribute<T3>)]
    pub tribute: Tribute,
}

pub struct TributeSummon {
    position: Position,
}

impl TributeSummon {
    pub fn new(position: Position) -> Self {
        TributeSummon { position }
    }
}

pub struct Tribute(pub CardID);
impl NonEmptyInput for Tribute {}
create_valid_identification!(ValidTribute, ValidCardID<CardIn<MonsterZone>>);
impl<F> ValidTribute<F> {
    pub(crate) fn new(card_id: ValidCardID<CardIn<MonsterZone>>) -> Self {
        ValidTribute(card_id, std::marker::PhantomData::default())
    }
}

impl ActionIdentifier for TributeSummon {
    fn action_id() -> ActionID {
        ActionID::new("tribute_summon")
    }
}
impl ValidAction<MainStep, TributeSummonInput> for TributeSummon {
    type Filter = (
        Condition<PlayerID, For<ActivePlayer>>,
        Condition<(ValidPlayerID<ActivePlayer>, CardID), CardIn<HandZone>>,
        Condition<
            (ValidPlayerID<ActivePlayer>, SlotID),
            With<(Free<MonsterSlot>, In<MonsterZone>)>,
        >,
        Condition<(ValidPlayerID<ActivePlayer>, Tribute), CardIn<MonsterZone>>,
    );
    type Output = (ValidPlayerID<ActivePlayer>, ValidTribute<()>);
    fn with_valid_input(
        self,
        state: MainStep,
        valid: <Self::Filter as StateFilter<MainStep, TributeSummonInput>>::ValidOutput,
    ) -> Self::Output {
        todo!()
    }
}
