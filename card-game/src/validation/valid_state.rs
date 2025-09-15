use crate::{
    cards::CardID,
    identifications::{PlayerID, ValidPlayerID},
    validation::ValidAction,
    zones::{ValidCardID, Zone},
};

pub struct ValidState<'a, State, Filter: StateFilter> {
    state: State,
    value: Filter::Value,
    _p: std::marker::PhantomData<&'a Filter>,
}
pub trait StateFilter {
    type Value;
    type Valid<'a>;
}
impl<F0: StateFilter, F1: StateFilter> StateFilter for (F0, F1) {
    type Value = (F0::Value, F1::Value);
    type Valid = (F0::Valid, F1::Valid);
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
impl<'a, State, Filter: StateFilter> ValidState<'a, State, Filter> {
    pub fn state(&self) -> &State {
        &self.state
    }
}
