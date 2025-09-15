use crate::{
    validation::{CardIn, ValidAction},
    zones::Zone,
};

pub struct TransferZone;

impl<S, Z: Zone> ValidAction for TransferZone {
    type State = S;
    type Filter = CardIn<Z>;
    type Output = Self::State;
    fn with_valid_input(
        self,
        state: Self::State,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter>::Valid<'_>,
    ) -> Self::Output {
    }
}
