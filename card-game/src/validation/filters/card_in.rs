use crate::{
    cards::CardID,
    identifications::{PlayerID, ValidPlayerID},
    zones::{ValidCardID, Zone},
};

pub struct CardIn<Z: Zone>(std::marker::PhantomData<Z>);
impl<Z: Zone> StateFilter for CardIn<Z> {
    type Value = (PlayerID, CardID);
    type Valid<'a> = (ValidPlayerID<'a>, ValidCardID<'a, Z>);
}
impl<'a, State, Z: Zone> ValidState<'a, State, CardIn<Z>> {
    pub fn execute<Action: ValidAction<Filter = CardIn<Z>>>(
        mut self,
        valid_action: Action,
    ) -> Action::Output {
        valid_action.with_valid_input(
            self.state,
            (
                ValidPlayerID::new(self.value.0),
                ValidCardID::new(self.value.1),
            ),
        )
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn valid_player_id(&self) -> ValidPlayerID<'_> {
        ValidPlayerID::new(self.value.0)
    }
    pub fn valid_card_id(&self) -> ValidCardID<'_, Z> {
        ValidCardID::new(self.value.1)
    }
}
