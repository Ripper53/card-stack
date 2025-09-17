use crate::{
    cards::CardID,
    identifications::{PlayerID, ValidPlayerID},
    validation::ValidAction,
    zones::{ValidCardID, Zone},
};

pub trait StateFilter<State>: Sized {
    type Input;
    type ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput>;
}
impl<State, F0: StateFilter<State>, F1: StateFilter<State, Input = F0::ValidOutput>>
    StateFilter<State> for (F0, F1)
{
    type Input = F0::Input;
    type ValidOutput = F1::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| F1::filter(state, v))
    }
}
impl<
    State,
    F0: StateFilter<State>,
    F1: StateFilter<State, Input = F0::ValidOutput>,
    F2: StateFilter<State, Input = F1::ValidOutput>,
> StateFilter<State> for (F0, F1, F2)
{
    type Input = F0::Input;
    type ValidOutput = F2::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value)
            .and_then(|v| F1::filter(state, v).and_then(|v| F2::filter(state, v)))
    }
}
impl<
    State,
    F0: StateFilter<State>,
    F1: StateFilter<State, Input = F0::ValidOutput>,
    F2: StateFilter<State, Input = F1::ValidOutput>,
    F3: StateFilter<State, Input = F2::ValidOutput>,
> StateFilter<State> for (F0, F1, F2, F3)
{
    type Input = F0::Input;
    type ValidOutput = F3::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| {
            F1::filter(state, v)
                .and_then(|v| F2::filter(state, v).and_then(|v| F3::filter(state, v)))
        })
    }
}
impl<
    State,
    F0: StateFilter<State>,
    F1: StateFilter<State, Input = F0::ValidOutput>,
    F2: StateFilter<State, Input = F1::ValidOutput>,
    F3: StateFilter<State, Input = F2::ValidOutput>,
    F4: StateFilter<State, Input = F3::ValidOutput>,
> StateFilter<State> for (F0, F1, F2, F3, F4)
{
    type Input = F0::Input;
    type ValidOutput = F4::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| {
            F1::filter(state, v).and_then(|v| {
                F2::filter(state, v)
                    .and_then(|v| F3::filter(state, v).and_then(|v| F4::filter(state, v)))
            })
        })
    }
}
