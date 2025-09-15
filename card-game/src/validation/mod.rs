mod actions;
pub mod filters;
mod valid_state;
pub use actions::*;
pub use valid_state::*;

use crate::zones::{ValidCardID, Zone, ZoneContext};

pub struct Validator<State, Filter: StateFilter> {
    state: State,
    value: Filter::Value,
    _p: std::marker::PhantomData<Filter>,
}
impl<State, Z: Zone> Validator<State, CardIn<Z>> {
    pub fn new(
        state: State,
        get_zone: for<'a> fn(&'a State) -> &'a Z,
        get_valid_card_id: impl for<'a> FnOnce(ZoneContext<'a, Z>) -> Option<ValidCardID<'a, Z>>,
    ) -> Result<Self, ZoneCardValidationError> {
        let zone = get_zone(&state);
        let player_id = zone.player_id();
        let zone = ZoneContext::new(zone);
        if let Some(card_id) = get_valid_card_id(zone).map(|zone_card_id| zone_card_id.card_id()) {
            Ok(Validator {
                state,
                value: (player_id, card_id),
                _p: std::marker::PhantomData::default(),
            })
        } else {
            Err(ZoneCardValidationError)
        }
    }
    pub fn execute<'a, R>(self, f: impl FnOnce(ValidState<'a, State, CardIn<Z>>) -> R) -> R
    where
        Z: 'a,
    {
        f(ValidState::new(self.state, self.value))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("failed to validate card exists in zone")]
pub struct ZoneCardValidationError;
