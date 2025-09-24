mod actions;
mod condition;
mod state_filter;
pub use actions::*;
pub use condition::*;
pub use state_filter::*;

pub struct Validator<State, Input: StateFilterInput, Filter: StateFilter<State, Input>> {
    state: State,
    value: Filter::ValidOutput,
    _p: std::marker::PhantomData<(Input, Filter)>,
}

impl<State, Input: StateFilterInput, Filter: StateFilter<State, Input>>
    Validator<State, Input, Filter>
{
    pub fn try_new(state: State, input: Input) -> Option<Self> {
        let value = Filter::filter(&state, input)?;
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
