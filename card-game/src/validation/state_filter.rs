pub trait StateFilter<State, Input>: Sized {
    type ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput>;
}
impl<State, Input, F0: StateFilter<State, Input>, F1: StateFilter<State, F0::ValidOutput>>
    StateFilter<State, Input> for (F0, F1)
{
    type ValidOutput = F1::ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| F1::filter(state, v))
    }
}
impl<
    State,
    Input,
    F0: StateFilter<State, Input>,
    F1: StateFilter<State, F0::ValidOutput>,
    F2: StateFilter<State, F1::ValidOutput>,
> StateFilter<State, Input> for (F0, F1, F2)
{
    type ValidOutput = F2::ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value)
            .and_then(|v| F1::filter(state, v).and_then(|v| F2::filter(state, v)))
    }
}
impl<
    State,
    Input,
    F0: StateFilter<State, Input>,
    F1: StateFilter<State, F0::ValidOutput>,
    F2: StateFilter<State, F1::ValidOutput>,
    F3: StateFilter<State, F2::ValidOutput>,
> StateFilter<State, Input> for (F0, F1, F2, F3)
{
    type ValidOutput = F3::ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| {
            F1::filter(state, v)
                .and_then(|v| F2::filter(state, v).and_then(|v| F3::filter(state, v)))
        })
    }
}
impl<
    State,
    Input,
    F0: StateFilter<State, Input>,
    F1: StateFilter<State, F0::ValidOutput>,
    F2: StateFilter<State, F1::ValidOutput>,
    F3: StateFilter<State, F2::ValidOutput>,
    F4: StateFilter<State, F3::ValidOutput>,
> StateFilter<State, Input> for (F0, F1, F2, F3, F4)
{
    type ValidOutput = F4::ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| {
            F1::filter(state, v).and_then(|v| {
                F2::filter(state, v)
                    .and_then(|v| F3::filter(state, v).and_then(|v| F4::filter(state, v)))
            })
        })
    }
}
