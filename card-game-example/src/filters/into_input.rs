use card_game::validation::{StateFilter, StateFilterInput};

use crate::filters::FilterInput;

pub struct IntoInput;

impl<State, T: StateFilterInput> StateFilter<State, T> for IntoInput {
    type ValidOutput = FilterInput<T>;
    type Error = std::convert::Infallible;
    fn filter(_state: &State, value: T) -> Result<Self::ValidOutput, Self::Error> {
        Ok(FilterInput(value))
    }
}
