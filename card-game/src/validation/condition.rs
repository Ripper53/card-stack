use crate::validation::StateFilterInput;

pub struct Condition<Input: StateFilterInput, Filter>(std::marker::PhantomData<(Input, Filter)>);
