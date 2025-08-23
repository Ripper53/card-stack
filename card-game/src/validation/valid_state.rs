use crate::{
    cards::CardID,
    identifications::{PlayerID, ValidPlayerID},
    zones::{ValidCardID, Zone},
};

pub struct ValidState<'a, State, Filter: StateFilter> {
    state: State,
    value: Filter::Value,
    _p: std::marker::PhantomData<&'a Filter>,
}
pub trait StateFilter {
    type Value;
}
impl<F0: StateFilter, F1: StateFilter> StateFilter for (F0, F1) {
    type Value = (F0::Value, F1::Value);
}
impl<'a, State, Filter: StateFilter> ValidState<'a, State, Filter> {
    pub(crate) fn new(state: State, value: Filter::Value) -> Self {
        ValidState {
            state,
            value,
            _p: std::marker::PhantomData::default(),
        }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
}
impl<'a, State> ValidState<'a, State, PlayerID> {
    pub fn take_all<'id>(self) -> (State, ValidPlayerID<'id>) {
        (self.state, ValidPlayerID::new(self.value))
    }
    pub fn player_id(&self) -> ValidPlayerID<'_> {
        ValidPlayerID::new(self.value)
    }
}
pub struct CardIn<Z: Zone>(std::marker::PhantomData<Z>);
impl<Z: Zone> StateFilter for CardIn<Z> {
    type Value = (PlayerID, CardID);
}
impl<'a, State, Z: Zone> ValidState<'a, State, CardIn<Z>> {
    pub fn take_all<'id>(self) -> (State, ValidPlayerID<'id>, ValidCardID<'id, Z>) {
        (
            self.state,
            ValidPlayerID::new(self.value.0),
            ValidCardID::new(self.value.1),
        )
    }
    pub fn valid_player_id(&self) -> ValidPlayerID<'_> {
        ValidPlayerID::new(self.value.0)
    }
    pub fn valid_card_id(&self) -> ValidCardID<'_, Z> {
        ValidCardID::new(self.value.1)
    }
}
