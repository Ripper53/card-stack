pub mod transfer;

pub trait ValidAction {
    type State;
    type Filter: StateFilter;
    type Output;
    fn with_valid_input(
        self,
        state: Self::State,
        valid: <Self::Filter as StateFilter>::Valid<'_>,
    ) -> Self::Output;
}
