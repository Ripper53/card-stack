use crate::validation::StateFilter;

pub trait ValidAction<State> {
    type Filter: StateFilter<State>;
    type Output;
    fn with_valid_input(
        self,
        state: State,
        valid: <Self::Filter as StateFilter<State>>::ValidOutput,
    ) -> Self::Output;
}
