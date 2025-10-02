use card_game::{
    StateFilterInput,
    cards::CardID,
    identifications::{PlayerID, SourceCardID, ValidCardID, ValidPlayerID},
    validation::{StateFilterCombination, StateFilterInput, StateFilterInputConversion},
};

mod any;
mod card_in;
mod r#for;
mod free;
mod r#in;
mod of_type;
mod slot;
mod with;
pub use any::*;
pub use card_in::*;
pub use r#for::*;
pub use free::*;
pub use r#in::*;
pub use of_type::*;
pub use slot::*;
pub use with::*;

use crate::{
    identifications::ValidSlotID,
    valid_actions::{Tribute, ValidTribute},
    zones::SlotID,
};

/*#[derive(StateFilterInput)]
#[state_filter_input(
    remainder_type = FilterInput<()>,
    remainder = FilterInput(()),
)]*/
pub struct FilterInput<T>(pub T);
impl<T> StateFilterInput for FilterInput<T> {}

impl<T> StateFilterInputConversion<FilterInput<T>> for FilterInput<T> {
    type Remainder = FilterInput<()>;
    fn split_take(self) -> (FilterInput<T>, Self::Remainder) {
        (self, FilterInput(()))
    }
}
impl<T> StateFilterCombination<T> for FilterInput<()> {
    type Combined = FilterInput<T>;
    fn combine(self, value: T) -> Self::Combined {
        FilterInput(value)
    }
}
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

impl<F0, F1> StateFilterInputConversion<FilterInput<(ValidPlayerID<F0>, SlotID)>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>
{
    type Remainder = FilterInput<ValidCardID<F1>>;
    fn split_take(self) -> (FilterInput<(ValidPlayerID<F0>, SlotID)>, Self::Remainder) {
        (FilterInput((self.0.0, self.0.2)), FilterInput(self.0.1))
    }
}

impl<F0, F1> StateFilterCombination<FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>>
    for FilterInput<SlotID>
{
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>;
    fn combine(self, value: FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>) -> Self::Combined {
        FilterInput((value.0.0, value.0.1, self.0))
    }
}

impl<F0, F1, F2> StateFilterCombination<FilterInput<(ValidPlayerID<F0>, ValidSlotID<F2>)>>
    for FilterInput<ValidCardID<F1>>
{
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, ValidSlotID<F2>)>;
    fn combine(self, value: FilterInput<(ValidPlayerID<F0>, ValidSlotID<F2>)>) -> Self::Combined {
        FilterInput((value.0.0, self.0, value.0.1))
    }
}

impl StateFilterInputConversion<FilterInput<PlayerID>> for FilterInput<(PlayerID, CardID, SlotID)> {
    type Remainder = FilterInput<(CardID, SlotID)>;
    fn split_take(self) -> (FilterInput<PlayerID>, Self::Remainder) {
        (FilterInput(self.0.0), FilterInput((self.0.1, self.0.2)))
    }
}

impl<F> StateFilterCombination<ValidPlayerID<F>> for FilterInput<(CardID, SlotID)> {
    type Combined = FilterInput<(ValidPlayerID<F>, CardID, SlotID)>;
    fn combine(self, value: ValidPlayerID<F>) -> Self::Combined {
        FilterInput((value, self.0.0, self.0.1))
    }
}

impl<F0, F1> StateFilterInputConversion<FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>
{
    type Remainder = FilterInput<SlotID>;
    fn split_take(
        self,
    ) -> (
        FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>,
        Self::Remainder,
    ) {
        (FilterInput((self.0.0, self.0.1)), FilterInput(self.0.2))
    }
}

impl<F0, F1> StateFilterInputConversion<(ValidPlayerID<F0>, SlotID)>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>
{
    type Remainder = FilterInput<ValidCardID<F1>>;
    fn split_take(self) -> ((ValidPlayerID<F0>, SlotID), Self::Remainder) {
        ((self.0.0, self.0.2), FilterInput(self.0.1))
    }
}

impl<F> StateFilterInputConversion<FilterInput<(ValidPlayerID<F>, CardID)>>
    for FilterInput<(ValidPlayerID<F>, CardID, SlotID)>
{
    type Remainder = FilterInput<SlotID>;
    fn split_take(self) -> (FilterInput<(ValidPlayerID<F>, CardID)>, Self::Remainder) {
        (FilterInput((self.0.0, self.0.1)), FilterInput(self.0.2))
    }
}

impl<F> StateFilterInputConversion<FilterInput<ValidPlayerID<F>>>
    for FilterInput<(ValidPlayerID<F>, CardID, SlotID)>
{
    type Remainder = FilterInput<(CardID, SlotID)>;
    fn split_take(self) -> (FilterInput<ValidPlayerID<F>>, Self::Remainder) {
        (FilterInput(self.0.0), FilterInput((self.0.1, self.0.2)))
    }
}

impl StateFilterInputConversion<FilterInput<PlayerID>>
    for FilterInput<(PlayerID, CardID, SlotID, Tribute)>
{
    type Remainder = FilterInput<(CardID, SlotID, Tribute)>;
    fn split_take(self) -> (FilterInput<PlayerID>, Self::Remainder) {
        (
            FilterInput(self.0.0),
            FilterInput((self.0.1, self.0.2, self.0.3)),
        )
    }
}

impl<F> StateFilterCombination<ValidPlayerID<F>> for FilterInput<(CardID, SlotID, Tribute)> {
    type Combined = FilterInput<(ValidPlayerID<F>, CardID, SlotID, Tribute)>;
    fn combine(self, value: ValidPlayerID<F>) -> Self::Combined {
        FilterInput((value, self.0.0, self.0.1, self.0.2))
    }
}

impl<F> StateFilterInputConversion<FilterInput<(ValidPlayerID<F>, CardID)>>
    for FilterInput<(ValidPlayerID<F>, CardID, SlotID, Tribute)>
{
    type Remainder = FilterInput<(SlotID, Tribute)>;
    fn split_take(self) -> (FilterInput<(ValidPlayerID<F>, CardID)>, Self::Remainder) {
        (
            FilterInput((self.0.0, self.0.1)),
            FilterInput((self.0.2, self.0.3)),
        )
    }
}

impl<F0, F1> StateFilterCombination<(ValidPlayerID<F0>, ValidCardID<F1>)>
    for FilterInput<(SlotID, Tribute)>
{
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID, Tribute)>;
    fn combine(self, value: (ValidPlayerID<F0>, ValidCardID<F1>)) -> Self::Combined {
        FilterInput((value.0, value.1, self.0.0, self.0.1))
    }
}

impl<F0, F1> StateFilterInputConversion<FilterInput<(ValidPlayerID<F0>, SlotID)>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID, Tribute)>
{
    type Remainder = FilterInput<(ValidCardID<F1>, Tribute)>;
    fn split_take(self) -> (FilterInput<(ValidPlayerID<F0>, SlotID)>, Self::Remainder) {
        (
            FilterInput((self.0.0, self.0.2)),
            FilterInput((self.0.1, self.0.3)),
        )
    }
}

impl<F0, F1, F2> StateFilterCombination<(ValidPlayerID<F0>, ValidSlotID<F1>)>
    for FilterInput<(ValidCardID<F2>, Tribute)>
{
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F2>, ValidSlotID<F1>, Tribute)>;
    fn combine(self, value: (ValidPlayerID<F0>, ValidSlotID<F1>)) -> Self::Combined {
        FilterInput((value.0, self.0.0, value.1, self.0.1))
    }
}
impl<F0, F1, F2> StateFilterCombination<FilterInput<(ValidPlayerID<F0>, ValidSlotID<F2>)>>
    for FilterInput<(ValidCardID<F1>, Tribute)>
{
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, ValidSlotID<F2>, Tribute)>;
    fn combine(self, value: FilterInput<(ValidPlayerID<F0>, ValidSlotID<F2>)>) -> Self::Combined {
        FilterInput((value.0.0, self.0.0, value.0.1, self.0.1))
    }
}

impl<F0, F1, F2> StateFilterInputConversion<FilterInput<(ValidPlayerID<F0>, Tribute)>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, ValidSlotID<F2>, Tribute)>
{
    type Remainder = FilterInput<(ValidCardID<F1>, ValidSlotID<F2>)>;
    fn split_take(self) -> (FilterInput<(ValidPlayerID<F0>, Tribute)>, Self::Remainder) {
        (
            FilterInput((self.0.0, self.0.3)),
            FilterInput((self.0.1, self.0.2)),
        )
    }
}

impl<F0, F1, F2> StateFilterCombination<(ValidPlayerID<F0>, ValidTribute)>
    for FilterInput<(ValidCardID<F1>, ValidSlotID<F2>)>
{
    type Combined = FilterInput<(
        ValidPlayerID<F0>,
        ValidCardID<F1>,
        ValidSlotID<F2>,
        ValidTribute,
    )>;
    fn combine(self, value: (ValidPlayerID<F0>, ValidTribute)) -> Self::Combined {
        FilterInput((value.0, self.0.0, self.0.1, value.1))
    }
}

impl<F> StateFilterCombination<ValidPlayerID<F>> for FilterInput<(SourceCardID, SlotID)> {
    type Combined = FilterInput<(ValidPlayerID<F>, SourceCardID, SlotID)>;
    fn combine(self, value: ValidPlayerID<F>) -> Self::Combined {
        FilterInput((value, self.0.0, self.0.1))
    }
}

impl<F> StateFilterInputConversion<FilterInput<ValidPlayerID<F>>>
    for FilterInput<(ValidPlayerID<F>, SourceCardID, CardID)>
{
    type Remainder = FilterInput<(SourceCardID, CardID)>;
    fn split_take(self) -> (FilterInput<ValidPlayerID<F>>, Self::Remainder) {
        (FilterInput(self.0.0), FilterInput((self.0.1, self.0.2)))
    }
}

impl StateFilterInputConversion<SourceCardID> for FilterInput<(PlayerID, SourceCardID)> {
    type Remainder = FilterInput<PlayerID>;
    fn split_take(self) -> (SourceCardID, Self::Remainder) {
        (self.0.1, FilterInput(self.0.0))
    }
}

impl<F> StateFilterCombination<ValidCardID<F>> for FilterInput<PlayerID> {
    type Combined = FilterInput<(PlayerID, ValidCardID<F>)>;
    fn combine(self, value: ValidCardID<F>) -> Self::Combined {
        FilterInput((self.0, value))
    }
}

impl<F> StateFilterCombination<FilterInput<(PlayerID, ValidCardID<F>)>> for FilterInput<SlotID> {
    type Combined = FilterInput<(PlayerID, ValidCardID<F>, SlotID)>;
    fn combine(self, value: FilterInput<(PlayerID, ValidCardID<F>)>) -> Self::Combined {
        FilterInput((value.0.0, value.0.1, self.0))
    }
}

impl<F> StateFilterInputConversion<FilterInput<PlayerID>>
    for FilterInput<(PlayerID, ValidCardID<F>, SlotID)>
{
    type Remainder = FilterInput<(ValidCardID<F>, SlotID)>;
    fn split_take(self) -> (FilterInput<PlayerID>, Self::Remainder) {
        (FilterInput(self.0.0), FilterInput((self.0.1, self.0.2)))
    }
}

impl<F0, F1> StateFilterCombination<ValidPlayerID<F0>> for FilterInput<(ValidCardID<F1>, SlotID)> {
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>;
    fn combine(self, value: ValidPlayerID<F0>) -> Self::Combined {
        FilterInput((value, self.0.0, self.0.1))
    }
}

impl<F> StateFilterCombination<ValidCardID<F>> for FilterInput<(PlayerID, SlotID)> {
    type Combined = FilterInput<(PlayerID, ValidCardID<F>, SlotID)>;
    fn combine(self, value: ValidCardID<F>) -> Self::Combined {
        FilterInput((self.0.0, value, self.0.1))
    }
}

impl<F0, F1> StateFilterInputConversion<FilterInput<ValidPlayerID<F0>>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>
{
    type Remainder = FilterInput<(ValidCardID<F1>, SlotID)>;
    fn split_take(self) -> (FilterInput<ValidPlayerID<F0>>, Self::Remainder) {
        (FilterInput(self.0.0), FilterInput((self.0.1, self.0.2)))
    }
}

impl<F0, F1, F2> StateFilterCombination<(ValidPlayerID<F0>, ValidSlotID<F2>)>
    for FilterInput<ValidCardID<F1>>
{
    type Combined = FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, ValidSlotID<F2>)>;
    fn combine(self, value: (ValidPlayerID<F0>, ValidSlotID<F2>)) -> Self::Combined {
        FilterInput((value.0, self.0, value.1))
    }
}

/*impl<F0, F1> StateFilterInputConversion<FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>>
    for FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>, SlotID)>
{
    type Remainder = FilterInput<SlotID>;
    fn split_take(
        self,
    ) -> (
        FilterInput<(ValidPlayerID<F0>, ValidCardID<F1>)>,
        Self::Remainder,
    ) {
        (FilterInput((self.0, self.1)), FilterInput(self.2))
    }
}*/
