use crate::validation::StateFilter;

pub struct Condition<Input, Filter>(std::marker::PhantomData<(Input, Filter)>);
