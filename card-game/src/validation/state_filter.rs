pub trait StateFilter<State>: Sized {
    type Input;
    type ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput>;
}
impl<State, F0: StateFilter<State>, F1: StateFilter<State>> StateFilter<State> for (F0, F1)
where
    F0::ValidOutput: StateFilterInput<F1::Input>,
    F1::ValidOutput:
        StateFilterCombination<<F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder>,
{
    type Input = F0::Input;
    type ValidOutput = <F1::ValidOutput as StateFilterCombination<
        <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
    >>::Combined;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| {
            let (input, remainder) = v.split_take();
            F1::filter(state, input).map(|v| v.combine(remainder))
        })
    }
}
impl<State, F0: StateFilter<State>, F1: StateFilter<State>, F2: StateFilter<State>>
    StateFilter<State> for (F0, F1, F2)
where
    F0::ValidOutput: StateFilterInput<F1::Input>,
    F1::ValidOutput:
        StateFilterCombination<<F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder>,
    <F1::ValidOutput as StateFilterCombination<
        <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
    >>::Combined: StateFilterInput<F2::Input>,
    F2::ValidOutput: StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >>::Combined as StateFilterInput<F2::Input>>::Remainder,
    >,
{
    type Input = F0::Input;
    type ValidOutput = <F2::ValidOutput as StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >>::Combined as StateFilterInput<F2::Input>>::Remainder,
    >>::Combined;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value)
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| v.combine(remainder))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| v.combine(remainder))
            })
    }
}
impl<
    State,
    F0: StateFilter<State>,
    F1: StateFilter<State>,
    F2: StateFilter<State>,
    F3: StateFilter<State>,
> StateFilter<State> for (F0, F1, F2, F3)
where
    F0::ValidOutput: StateFilterInput<F1::Input>,
    F1::ValidOutput:
        StateFilterCombination<<F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder>,
    <F1::ValidOutput as StateFilterCombination<
        <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
    >>::Combined: StateFilterInput<F2::Input>,
    F2::ValidOutput: StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >>::Combined as StateFilterInput<F2::Input>>::Remainder,
    >,
    <F2::ValidOutput as StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >>::Combined as StateFilterInput<F2::Input>>::Remainder,
    >>::Combined: StateFilterInput<F3::Input>,
    F3::ValidOutput: StateFilterCombination<
        <<F2::ValidOutput as StateFilterCombination<
            <<F1::ValidOutput as StateFilterCombination<
                <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
            >>::Combined as StateFilterInput<F2::Input>>::Remainder,
        >>::Combined as StateFilterInput<F3::Input>>::Remainder,
    >,
{
    type Input = F0::Input;
    type ValidOutput = <F3::ValidOutput as StateFilterCombination<
        <<F2::ValidOutput as StateFilterCombination<
            <<F1::ValidOutput as StateFilterCombination<
                <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
            >>::Combined as StateFilterInput<F2::Input>>::Remainder,
        >>::Combined as StateFilterInput<F3::Input>>::Remainder,
    >>::Combined;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value)
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| v.combine(remainder))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| v.combine(remainder))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| v.combine(remainder))
            })
    }
}

pub trait StateFilterInput<T>: Sized {
    type Remainder;
    fn new(input: T, remainder: Self::Remainder) -> Self;
    fn split_take(self) -> (T, Self::Remainder);
}
impl<T> StateFilterInput<T> for T {
    type Remainder = ();
    fn new(input: T, (): ()) -> Self {
        input
    }
    fn split_take(self) -> (T, Self::Remainder) {
        (self, ())
    }
}
impl<T, R> StateFilterInput<T> for (R, T) {
    type Remainder = R;
    fn new(input: T, remainder: Self::Remainder) -> Self {
        (remainder, input)
    }
    fn split_take(self) -> (T, Self::Remainder) {
        (self.1, self.0)
    }
}

pub trait StateFilterCombination<T>: Sized {
    type Combined;
    fn combine(self, value: T) -> Self::Combined;
}

impl<T, U> StateFilterCombination<T> for U {
    type Combined = (Self, T);
    fn combine(self, value: T) -> Self::Combined {
        (self, value)
    }
}
