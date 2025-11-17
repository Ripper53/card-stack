use card_game::{
    cards::CardID,
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
};
use state_validation::{StateFilterInput, StateFilterInputCombination, StateFilterInputConversion};

use crate::{identifications::ValidSlotID, zones::SlotID};

#[derive(StateFilterInput)]
pub struct NormalSummonInput {
    #[conversion(T0 = ValidPlayerID<T0>)]
    pub player_id: PlayerID,
    #[conversion(T1 = ValidCardID<T1>)]
    pub card_id: CardID,
    #[conversion(T2 = ValidSlotID<T2>)]
    pub slot_id: SlotID,
}

/*impl StateFilterInputCombination<PlayerID> for NormalSummonInput {
    type Remainder = A;
    fn split_take(self) -> (PlayerID, Self::Remainder) {
        (
            self.player_id,
            A {
                card_id: self.card_id,
                slot_id: self.slot_id,
            },
        )
    }
}

pub struct B<F> {
    player_id: ValidPlayerID<F>,
    card_id: CardID,
    slot_id: SlotID,
}
impl<F> StateFilterInputCombination<ValidPlayerID<F>> for A {
    type Combined = B<F>;
    fn combine(self, player_id: ValidPlayerID<F>) -> Self::Combined {
        B {
            player_id,
            card_id: self.card_id,
            slot_id: self.slot_id,
        }
    }
}
pub struct C {
    slot_id: SlotID,
}
impl<F> StateFilterInputConversion<(ValidPlayerID<F>, CardID)> for B<F> {
    type Remainder = C;
    fn split_take(self) -> ((ValidPlayerID<F>, CardID), Self::Remainder) {
        (
            (self.player_id, self.card_id),
            C {
                slot_id: self.slot_id,
            },
        )
    }
}

pub struct D<F0, F1> {
    player_id: ValidPlayerID<F0>,
    card_id: ValidCardID<F1>,
    slot_id: SlotID,
}
impl<F0, F1> StateFilterInputCombination<(ValidPlayerID<F0>, ValidCardID<F1>)> for C {
    type Combined = D<F0, F1>;
    fn combine(self, (player_id, card_id): (ValidPlayerID<F0>, ValidCardID<F1>)) -> Self::Combined {
        D {
            player_id,
            card_id,
            slot_id: self.slot_id,
        }
    }
}

pub struct E {
    slot_id: SlotID,
}
impl<F0, F1> StateFilterInputConversion<(ValidPlayerID<F0>, ValidCardID<F1>)> for D<F0, F1> {
    type Remainder = E;
    fn split_take(self) -> ((ValidPlayerID<F0>, ValidCardID<F1>), Self::Remainder) {
        (
            (self.player_id, self.card_id),
            E {
                slot_id: self.slot_id,
            },
        )
    }
}

impl<F0, F1> StateFilterInputCombination<(ValidPlayerID<F0>, ValidCardID<F1>)> for E {
    type Combined = D<F0, F1>;
    fn combine(self, (player_id, card_id): (ValidPlayerID<F0>, ValidCardID<F1>)) -> Self::Combined {
        D {
            player_id,
            card_id,
            slot_id: self.slot_id,
        }
    }
}

pub struct G<F> {
    card_id: ValidCardID<F>,
}
impl<F0, F1> StateFilterInputConversion<(ValidPlayerID<F0>, SlotID)> for D<F0, F1> {
    type Remainder = G<F1>;
    fn split_take(self) -> ((ValidPlayerID<F0>, SlotID), Self::Remainder) {
        (
            (self.player_id, self.slot_id),
            G {
                card_id: self.card_id,
            },
        )
    }
}

impl<F0, F1, F2> StateFilterInputCombination<(ValidPlayerID<F0>, ValidSlotID<F2>)> for G<F1> {
    type Combined = (ValidPlayerID<F0>, ValidCardID<F1>, ValidSlotID<F2>);
    fn combine(self, (player_id, slot_id): (ValidPlayerID<F0>, ValidSlotID<F2>)) -> Self::Combined {
        (player_id, self.card_id, slot_id)
    }
}*/
