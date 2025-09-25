use crate::validation::Condition;

pub trait StateFilter<State, Input: StateFilterInput>: Sized {
    type ValidOutput;
    fn filter(state: &State, value: Input) -> Option<Self::ValidOutput>;
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
> StateFilter<State, InitialInput> for (Condition<Input0, F0>, Condition<Input1, F1>)
where
    InitialInput: StateFilterInputConversion<Input0>,
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
{
    type ValidOutput = <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
        >>::Combined as StateFilterInputConversion<Input1>>::Remainder as
        StateFilterCombination<F1::ValidOutput>>::Combined;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    Input2: StateFilterInput,
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
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
    <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input2>,

    <<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder:
        StateFilterCombination<F2::ValidOutput>,
{
    type ValidOutput = 
    <<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined
    ;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    Input2: StateFilterInput,
    Input3: StateFilterInput,
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
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
    <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input2>,

    <<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder:
        StateFilterCombination<F2::ValidOutput>,
        
        
    <<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined: StateFilterInputConversion<Input3>,

    <<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder: StateFilterCombination<F3::ValidOutput>,
{
    type ValidOutput = 
    <<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined
    ;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    Input2: StateFilterInput,
    Input3: StateFilterInput,
    Input4: StateFilterInput,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
    F2: StateFilter<State, Input2>,
    F3: StateFilter<State, Input3>,
    F4: StateFilter<State, Input4>,
> StateFilter<State, InitialInput>
    for (
        Condition<Input0, F0>,
        Condition<Input1, F1>,
        Condition<Input2, F2>,
        Condition<Input3, F3>,
        Condition<Input4, F4>,
    )
where
    InitialInput: StateFilterInputConversion<Input0>,
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
    <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input2>,

    <<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder:
        StateFilterCombination<F2::ValidOutput>,
        
        
    <<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined: StateFilterInputConversion<Input3>,

    <<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder: StateFilterCombination<F3::ValidOutput>,

    <<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined: StateFilterInputConversion<Input4>,

    <<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder: StateFilterCombination<F4::ValidOutput>,
{
    type ValidOutput =
    <<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined
    ;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F4::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    Input2: StateFilterInput,
    Input3: StateFilterInput,
    Input4: StateFilterInput,
    Input5: StateFilterInput,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
    F2: StateFilter<State, Input2>,
    F3: StateFilter<State, Input3>,
    F4: StateFilter<State, Input4>,
    F5: StateFilter<State, Input5>,
> StateFilter<State, InitialInput>
    for (
        Condition<Input0, F0>,
        Condition<Input1, F1>,
        Condition<Input2, F2>,
        Condition<Input3, F3>,
        Condition<Input4, F4>,
        Condition<Input5, F5>,
    )
where
    InitialInput: StateFilterInputConversion<Input0>,
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
    <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input2>,

    <<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder:
        StateFilterCombination<F2::ValidOutput>,
        
        
    <<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined: StateFilterInputConversion<Input3>,

    <<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder: StateFilterCombination<F3::ValidOutput>,

    <<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined: StateFilterInputConversion<Input4>,

    <<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder: StateFilterCombination<F4::ValidOutput>,

    <<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined: StateFilterInputConversion<Input5>,

    <<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder: StateFilterCombination<F5::ValidOutput>,
{
    type ValidOutput = 
    <<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined
    ;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F4::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F5::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    Input2: StateFilterInput,
    Input3: StateFilterInput,
    Input4: StateFilterInput,
    Input5: StateFilterInput,
    Input6: StateFilterInput,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
    F2: StateFilter<State, Input2>,
    F3: StateFilter<State, Input3>,
    F4: StateFilter<State, Input4>,
    F5: StateFilter<State, Input5>,
    F6: StateFilter<State, Input6>,
> StateFilter<State, InitialInput>
    for (
        Condition<Input0, F0>,
        Condition<Input1, F1>,
        Condition<Input2, F2>,
        Condition<Input3, F3>,
        Condition<Input4, F4>,
        Condition<Input5, F5>,
        Condition<Input6, F6>,
    )
where
    InitialInput: StateFilterInputConversion<Input0>,
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
    <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input2>,

    <<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder:
        StateFilterCombination<F2::ValidOutput>,
        
        
    <<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined: StateFilterInputConversion<Input3>,

    <<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder: StateFilterCombination<F3::ValidOutput>,

    <<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined: StateFilterInputConversion<Input4>,

    <<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder: StateFilterCombination<F4::ValidOutput>,

    <<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined: StateFilterInputConversion<Input5>,

    <<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder: StateFilterCombination<F5::ValidOutput>,

    <<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined: StateFilterInputConversion<Input6>,

    <<<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined as StateFilterInputConversion<Input6>>::Remainder: StateFilterCombination<F6::ValidOutput>,
{
    type ValidOutput =
    <<<<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined as StateFilterInputConversion<Input6>>::Remainder as StateFilterCombination<F6::ValidOutput>>::Combined
    ;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F4::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F5::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F6::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}
impl<
    State,
    InitialInput: StateFilterInput,
    Input0: StateFilterInput,
    Input1: StateFilterInput,
    Input2: StateFilterInput,
    Input3: StateFilterInput,
    Input4: StateFilterInput,
    Input5: StateFilterInput,
    Input6: StateFilterInput,
    Input7: StateFilterInput,
    F0: StateFilter<State, Input0>,
    F1: StateFilter<State, Input1>,
    F2: StateFilter<State, Input2>,
    F3: StateFilter<State, Input3>,
    F4: StateFilter<State, Input4>,
    F5: StateFilter<State, Input5>,
    F6: StateFilter<State, Input6>,
    F7: StateFilter<State, Input7>,
> StateFilter<State, InitialInput>
    for (
        Condition<Input0, F0>,
        Condition<Input1, F1>,
        Condition<Input2, F2>,
        Condition<Input3, F3>,
        Condition<Input4, F4>,
        Condition<Input5, F5>,
        Condition<Input6, F6>,
        Condition<Input7, F7>,
    )
where
    InitialInput: StateFilterInputConversion<Input0>,
    <InitialInput as StateFilterInputConversion<Input0>>::Remainder:
        StateFilterCombination<F0::ValidOutput>,
    <<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input1>,
    <<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder:
        StateFilterCombination<F1::ValidOutput>,
    <<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined: StateFilterInputConversion<Input2>,

    <<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder:
        StateFilterCombination<F2::ValidOutput>,
        
        
    <<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined: StateFilterInputConversion<Input3>,

    <<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder: StateFilterCombination<F3::ValidOutput>,

    <<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined: StateFilterInputConversion<Input4>,

    <<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder: StateFilterCombination<F4::ValidOutput>,

    <<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined: StateFilterInputConversion<Input5>,

    <<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder: StateFilterCombination<F5::ValidOutput>,

    <<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined: StateFilterInputConversion<Input6>,

    <<<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined as StateFilterInputConversion<Input6>>::Remainder: StateFilterCombination<F6::ValidOutput>,

    <<<<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined as StateFilterInputConversion<Input6>>::Remainder as StateFilterCombination<F6::ValidOutput>>::Combined: StateFilterInputConversion<Input7>,

    <<<<<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined as StateFilterInputConversion<Input6>>::Remainder as StateFilterCombination<F6::ValidOutput>>::Combined as StateFilterInputConversion<Input7>>::Remainder: StateFilterCombination<F7::ValidOutput>,
{
    type ValidOutput=
    <<<<<<<<<<<<<<<<InitialInput as StateFilterInputConversion<Input0>>::Remainder as StateFilterCombination<
        F0::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input1>>::Remainder as StateFilterCombination<
        F1::ValidOutput,
    >>::Combined as StateFilterInputConversion<Input2>>::Remainder as
        StateFilterCombination<F2::ValidOutput>>::Combined as StateFilterInputConversion<Input3>>::Remainder as StateFilterCombination<F3::ValidOutput>>::Combined as StateFilterInputConversion<Input4>>::Remainder as StateFilterCombination<F4::ValidOutput>>::Combined as StateFilterInputConversion<Input5>>::Remainder as StateFilterCombination<F5::ValidOutput>>::Combined as StateFilterInputConversion<Input6>>::Remainder as StateFilterCombination<F6::ValidOutput>>::Combined as StateFilterInputConversion<Input7>>::Remainder as StateFilterCombination<F7::ValidOutput>>::Combined
        ;
    fn filter(state: &State, value: InitialInput) -> Option<Self::ValidOutput> {
        let (input, remainder) = value.split_take();
        F0::filter(state, input)
            .map(|v| remainder.combine(v))
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F4::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F5::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F6::filter(state, input).map(|v| remainder.combine(v))
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F7::filter(state, input).map(|v| remainder.combine(v))
            })
    }
}

pub trait StateFilterInput {}

pub trait StateFilterInputConversion<T>: Sized {
    type Remainder;
    fn split_take(self) -> (T, Self::Remainder);
}
impl<T: StateFilterInput> StateFilterInputConversion<T> for T {
    type Remainder = ();
    fn split_take(self) -> (T, Self::Remainder) {
        (self, ())
    }
}
impl<T0: StateFilterInput, T1: StateFilterInput> StateFilterInputConversion<T0> for (T0, T1) {
    type Remainder = (T1,);
    fn split_take(self) -> (T0, Self::Remainder) {
        (self.0, (self.1,))
    }
}
impl<T0: StateFilterInput, T1: StateFilterInput, T2: StateFilterInput>
    StateFilterInputConversion<T0> for (T0, T1, T2)
{
    type Remainder = (T1, T2);
    fn split_take(self) -> (T0, Self::Remainder) {
        (self.0, (self.1, self.2))
    }
}
impl<T0: StateFilterInput, T1: StateFilterInput, T2: StateFilterInput, T3: StateFilterInput>
    StateFilterInputConversion<T0> for (T0, T1, T2, T3)
{
    type Remainder = (T1, T2, T3);
    fn split_take(self) -> (T0, Self::Remainder) {
        (self.0, (self.1, self.2, self.3))
    }
}

pub trait StateFilterCombination<T>: Sized {
    type Combined;
    fn combine(self, value: T) -> Self::Combined;
}

impl<T: StateFilterInput> StateFilterCombination<T> for () {
    type Combined = T;
    fn combine(self, value: T) -> Self::Combined {
        value
    }
}
impl<T: StateFilterInput, U: StateFilterInput> StateFilterCombination<(T,)> for (U,) {
    type Combined = (U, T);
    fn combine(self, value: (T,)) -> Self::Combined {
        (self.0, value.0)
    }
}
impl<T: StateFilterInput, U: StateFilterInput> StateFilterCombination<(T,)> for (U, ()) {
    type Combined = (U, T);
    fn combine(self, value: (T,)) -> Self::Combined {
        (self.0, value.0)
    }
}
impl<T: StateFilterInput, U0: StateFilterInput, U1: StateFilterInput> StateFilterCombination<(T,)>
    for (U0, U1)
{
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
