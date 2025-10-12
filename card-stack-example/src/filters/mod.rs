use state_validation::{StateFilter, StateFilterInput};

pub struct Using<T>(std::marker::PhantomData<T>);

impl<State, T: StateFilterInput> StateFilter<State, T> for Using<T> {
    type ValidOutput = T;
    type Error = std::convert::Infallible;
    fn filter(_state: &State, value: T) -> Result<Self::ValidOutput, Self::Error> {
        Ok(value)
    }
}
