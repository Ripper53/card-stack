mod actions;
mod state_filter;
pub use actions::*;
pub use state_filter::*;

use crate::zones::{ValidCardID, Zone, ZoneContext};

pub struct Validator<
    State,
    Input,
    Filter: StateFilter<State, Input>,
    GetInput: for<'a> FnOnce(&'a State) -> Input,
> {
    state: State,
    value: Filter::ValidOutput,
    _p: std::marker::PhantomData<(Input, Filter, GetInput)>,
}

impl<State, Input, Filter: StateFilter<State, Input>, GetInput: for<'a> FnOnce(&'a State) -> Input>
    Validator<State, Input, Filter, GetInput>
{
    pub fn try_new(state: State, get_value: GetInput) -> Option<Self> {
        let value = get_value(&state);
        let value = Filter::filter(&state, value)?;
        Some(Validator {
            state,
            value,
            _p: std::marker::PhantomData::default(),
        })
    }
    pub fn execute<Action: ValidAction<State, Input, Filter = Filter>>(
        self,
        valid_action: Action,
    ) -> Action::Output {
        valid_action.with_valid_input(self.state, self.value)
    }
}
