use crate::validation::Condition;

pub trait StateFilter<State, Input>: Sized {
    type ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput>;
}
impl<
    State,
    InitialInput,
    Input0,
    Input1,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
> StateFilter<State, InitialInput> for (Condition<Input0, F0>, Condition<Input1, F1>)
where
    InitialInput: StateFilterInputConversion<Input0>,
    F0::ValidOutput:
        StateFilterCombination<<InitialInput as StateFilterInputConversion<Input0>>::Remainder>,
    <F0::ValidOutput as StateFilterCombination<
        <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
    >>::Combined: StateFilterInputConversion<Input1>,
    F1::ValidOutput: StateFilterCombination<
        <<F0::ValidOutput as StateFilterCombination<
            <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
    >,
{
    type ValidOutput = <F1::ValidOutput as StateFilterCombination<
        <<F0::ValidOutput as StateFilterCombination<
            <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
    >>::Combined;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| v.combine(remainder))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| v.combine(remainder))
            })
    }
}
impl<
    State,
    InitialInput,
    Input0,
    Input1,
    Input2,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
    F2: StateFilter<State, Input2>,
> StateFilter<State, InitialInput>
    for (
        Condition<Input0, F0>,
        Condition<Input1, F1>,
        Condition<Input2, F2>,
    )
where
    InitialInput: StateFilterInputConversion<Input0>,
    F0::ValidOutput:
        StateFilterCombination<<InitialInput as StateFilterInputConversion<Input0>>::Remainder>,
    <F0::ValidOutput as StateFilterCombination<
        <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
    >>::Combined: StateFilterInputConversion<Input1>,
    F1::ValidOutput: StateFilterCombination<
        <<F0::ValidOutput as StateFilterCombination<
            <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
    >,
    <F1::ValidOutput as StateFilterCombination<
        <<F0::ValidOutput as StateFilterCombination<
            <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
    >>::Combined: StateFilterInputConversion<Input2>,
    F2::ValidOutput: StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <<F0::ValidOutput as StateFilterCombination<
                <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
            >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input2>>::Remainder,
    >,
{
    type ValidOutput = <F2::ValidOutput as StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <<F0::ValidOutput as StateFilterCombination<
                <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
            >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input2>>::Remainder,
    >>::Combined;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| v.combine(remainder))
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
    InitialInput,
    Input0,
    Input1,
    Input2,
    Input3,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
    F2: StateFilter<State, Input2>,
    F3: StateFilter<State, Input3>,
> StateFilter<State, InitialInput>
    for (
        Condition<Input0, F0>,
        Condition<Input1, F1>,
        Condition<Input2, F2>,
        Condition<Input3, F3>,
    )
where
    InitialInput: StateFilterInputConversion<Input0>,
    F0::ValidOutput:
        StateFilterCombination<<InitialInput as StateFilterInputConversion<Input0>>::Remainder>,
    <F0::ValidOutput as StateFilterCombination<
        <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
    >>::Combined: StateFilterInputConversion<Input1>,
    F1::ValidOutput: StateFilterCombination<
        <<F0::ValidOutput as StateFilterCombination<
            <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
    >,
    <F1::ValidOutput as StateFilterCombination<
        <<F0::ValidOutput as StateFilterCombination<
            <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
    >>::Combined: StateFilterInputConversion<Input2>,
    F2::ValidOutput: StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <<F0::ValidOutput as StateFilterCombination<
                <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
            >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input2>>::Remainder,
    >,
    <F2::ValidOutput as StateFilterCombination<
        <<F1::ValidOutput as StateFilterCombination<
            <<F0::ValidOutput as StateFilterCombination<
                <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
            >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input2>>::Remainder,
    >>::Combined: StateFilterInputConversion<Input3>,
    F3::ValidOutput: StateFilterCombination<
        <<F2::ValidOutput as StateFilterCombination<
            <<F1::ValidOutput as StateFilterCombination<
                <<F0::ValidOutput as StateFilterCombination<
                    <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
                >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
            >>::Combined as StateFilterInputConversion<Input2>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input3>>::Remainder,
    >,
{
    type ValidOutput = <F3::ValidOutput as StateFilterCombination<
        <<F2::ValidOutput as StateFilterCombination<
            <<F1::ValidOutput as StateFilterCombination<
                <<F0::ValidOutput as StateFilterCombination<
                    <InitialInput as StateFilterInputConversion<Input0>>::Remainder,
                >>::Combined as StateFilterInputConversion<Input1>>::Remainder,
            >>::Combined as StateFilterInputConversion<Input2>>::Remainder,
        >>::Combined as StateFilterInputConversion<Input3>>::Remainder,
    >>::Combined;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| v.combine(remainder))
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
/*impl<State, F0: StateFilter<State>, F1: StateFilter<State>, F2: StateFilter<State>>
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
}*/

pub trait StateFilterInput {}

pub trait StateFilterInputConversion<T>: Sized {
    type Remainder;
    fn new(input: T, remainder: Self::Remainder) -> Self;
    fn split_take(self) -> (T, Self::Remainder);
}
impl<T> StateFilterInputConversion<T> for T {
    type Remainder = ();
    fn new(input: T, (): ()) -> Self {
        input
    }
    fn split_take(self) -> (T, Self::Remainder) {
        (self, ())
    }
}
impl<T, R> StateFilterInputConversion<T> for (R, T) {
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

impl<T> StateFilterCombination<T> for () {
    type Combined = T;
    fn combine(self, value: T) -> Self::Combined {
        value
    }
}
impl<T, U: StateFilterInput> StateFilterCombination<(T,)> for (U,) {
    type Combined = (U, T);
    fn combine(self, value: (T,)) -> Self::Combined {
        (self.0, value.0)
    }
}
impl<T, U: StateFilterInput> StateFilterCombination<(T,)> for (U, ()) {
    type Combined = (U, T);
    fn combine(self, value: (T,)) -> Self::Combined {
        (self.0, value.0)
    }
}
impl<T, U0: StateFilterInput, U1: StateFilterInput> StateFilterCombination<(T,)> for (U0, U1) {
    type Combined = (U0, U1, T);
    fn combine(self, value: (T,)) -> Self::Combined {
        (self.0, self.1, value.0)
    }
}
impl<U0: StateFilterInput, U1: StateFilterInput> StateFilterCombination<()> for (U0, U1) {
    type Combined = (U0, U1);
    fn combine(self, (): ()) -> Self::Combined {
        (self.0, self.1)
    }
}
