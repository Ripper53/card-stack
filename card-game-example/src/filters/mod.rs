use card_game::{
    StateFilterInput,
    cards::CardID,
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
    validation::{StateFilterCombination, StateFilterInput, StateFilterInputConversion},
};

mod card_in;
mod r#for;
mod free;
mod r#in;
mod of_type;
mod slot;
mod with;
pub use card_in::*;
pub use r#for::*;
pub use free::*;
pub use r#in::*;
pub use of_type::*;
pub use slot::*;
pub use with::*;

use crate::zones::SlotID;

#[derive(StateFilterInput)]
pub struct FilterInput<T>(pub T);

impl StateFilterInputConversion<FilterInput<(PlayerID, CardID)>>
    for FilterInput<(PlayerID, CardID, SlotID)>
{
    type Remainder = FilterInput<SlotID>;
    fn split_take(self) -> (FilterInput<(PlayerID, CardID)>, Self::Remainder) {
        (FilterInput((self.0.0, self.0.1)), FilterInput(self.0.2))
    }
}

impl<F0, F1> StateFilterCombination<(ValidPlayerID<F0>, ValidCardID<F1>)> for FilterInput<SlotID> {
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>;
    fn combine(self, value: (ValidPlayerID<F0>, ValidCardID<F1>)) -> Self::Combined {
        FilterInput((value.0, value.1, self.0))
    }
}

impl<F0, F1> StateFilterInputConversion<FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>
{
    type Remainder = SlotID;
    fn split_take(
        self,
    ) -> (
        FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>,
        Self::Remainder,
    ) {
        (FilterInput((self.0.0, self.0.1)), self.0.2)
    }
}

impl<T> StateFilterCombination<T> for FilterInput<T> {
    type Combined = Self;
    fn combine(self, value: T) -> Self::Combined {
        FilterInput(value)
    }
}
