use crate::{StateFilter, StateFilterInput};

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ActionID(&'static str);
impl ActionID {
    pub fn new(value: &'static str) -> Self {
        ActionID(value)
    }
}

pub trait ValidAction<State, Input: StateFilterInput> {
    type Filter: StateFilter<State, Input>;
    type Output;
    fn with_valid_input(
        self,
        state: State,
        valid: <Self::Filter as StateFilter<State, Input>>::ValidOutput,
    ) -> Self::Output;
    fn action_id() -> ActionID;
}
