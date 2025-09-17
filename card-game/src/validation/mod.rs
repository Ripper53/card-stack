mod actions;
mod state_filter;
pub use actions::*;
pub use state_filter::*;

use crate::zones::{ValidCardID, Zone, ZoneContext};

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
