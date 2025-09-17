mod actions;
pub mod filters;
mod state_filter;
pub use actions::*;
pub use state_filter::*;

use crate::{
    //validation::filters::CardIn,
    zones::{ValidCardID, Zone, ZoneContext},
};

pub struct Validator<State, Filter: StateFilter<State>> {
    state: State,
    value: Filter::ValidOutput,
    _p: std::marker::PhantomData<Filter>,
}

impl<State, Filter: StateFilter<State>> Validator<State, Filter> {
    pub fn try_new(
        state: State,
        get_value: impl for<'b> FnOnce(&'b State) -> Filter::Input,
    ) -> Option<Self> {
        let value = get_value(&state);
        let value = Filter::filter(&state, value)?;
        Some(Validator {
            state,
            value,
            _p: std::marker::PhantomData::default(),
        })
    }
    pub fn execute<Action: ValidAction<State, Filter = Filter>>(
        self,
        valid_action: Action,
    ) -> Action::Output {
        valid_action.with_valid_input(self.state, self.value)
    }
}

/*#[derive(thiserror::Error, Debug)]
#[error("failed to validate card exists in zone")]
pub struct ZoneCardValidationError;

macro_rules! impl_validator {
    ($($t: ident,)*) => {
        impl<'a, State, Z: Zone $(,$t: for<'a> StateFilter<State = State, Value = (CardIn<Z> as StateFilter)::Value> + 'static)*> Validator<'a, State, (CardIn<Z> $(,$t)*)> {
            pub fn try_new(
                state: State,
                get_zone: for<'b> fn(&'b State) -> &'b Z,
                get_valid_card_id: impl for<'b> FnOnce(ZoneContext<'b, Z, ($($t,)*)>) -> Option<ValidCardID<'b, (Z $(,$t)*)>>,
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
            pub fn execute<'b, R>(self, f: impl FnOnce(ValidState<'b, State, CardIn<(Z $(,$t)*)>>) -> R) -> R
            where
                Z: 'b,
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
impl_validator!(T0, T1, T2, T3,);*/
