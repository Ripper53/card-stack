mod actions;
pub mod filters;
mod valid_state;
pub use actions::*;
pub use valid_state::*;

use crate::{
    validation::filters::CardIn,
    zones::{ValidCardID, Zone, ZoneContext},
};

pub struct Validator<State, Filter: StateFilter> {
    state: State,
    value: Filter::Value,
    _p: std::marker::PhantomData<Filter>,
}

#[derive(thiserror::Error, Debug)]
#[error("failed to validate card exists in zone")]
pub struct ZoneCardValidationError;

macro_rules! impl_validator {
    ($($t: ident,)*) => {
        impl<State, Z: Zone + 'static $(,$t: 'static)*> Validator<State, CardIn<(Z $(,$t)*)>> {
            pub fn try_new(
                state: State,
                get_zone: for<'a> fn(&'a State) -> &'a Z,
                get_valid_card_id: impl for<'a> FnOnce(ZoneContext<'a, Z, ($($t,)*)>) -> Option<ValidCardID<'a, (Z $(,$t)*)>>,
            ) -> Result<Self, ZoneCardValidationError> {
                let zone = get_zone(&state);
                let player_id = zone.player_id();
                let zone = ZoneContext::<Z, ($($t,)*)>::new(zone);
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
            pub fn execute<'a, R>(self, f: impl FnOnce(ValidState<'a, State, CardIn<(Z $(,$t)*)>>) -> R) -> R
            where
                Z: 'a,
            {
                f(ValidState::new(self.state, self.value))
            }
        }
    };
}

impl_validator!();
impl_validator!(T0,);
impl_validator!(T0, T1,);
impl_validator!(T0, T1, T2,);
impl_validator!(T0, T1, T2, T3,);
