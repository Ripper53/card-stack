use crate::{StateFilter, StateFilterInput};

pub struct CollectedInputs<State, Inputs: Iterator>
where
    Inputs::Item: StateFilterInput,
{
    inputs: Inputs,
    _m: std::marker::PhantomData<State>,
}

impl<State, Inputs: Iterator> CollectedInputs<State, Inputs>
where
    Inputs::Item: StateFilterInput,
{
    pub fn new(inputs: Inputs) -> Self {
        CollectedInputs {
            inputs,
            _m: std::marker::PhantomData::default(),
        }
    }
    /// Do all the inputs pass the filter without error?
    pub fn fits_all<F: StateFilter<State, Inputs::Item>>(self, state: &State) -> bool {
        self.inputs
            .into_iter()
            .all(|input| F::filter(state, input).is_ok())
    }
    /// Do any of the inputs pass the filter without error?
    pub fn fits_any<F: StateFilter<State, Inputs::Item>>(self, state: &State) -> bool {
        self.inputs
            .into_iter()
            .any(|input| F::filter(state, input).is_ok())
    }
}

pub trait InputCollector<State, Inputs: Iterator>
where
    Inputs::Item: StateFilterInput,
{
    fn collect_inputs(state: &State, value: Inputs::Item) -> CollectedInputs<State, Inputs>;
}
