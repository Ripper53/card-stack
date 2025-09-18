pub trait StateFilter<State>: Sized {
    type Input;
    type ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput>;
}
impl<State, F0: StateFilter<State>, F1: StateFilter<State>> StateFilter<State> for (F0, F1)
where
    F0::ValidOutput: StateFilterInput<F1::Input>,
    F1::ValidOutput: StateFilterInput<
            F1::ValidOutput,
            Remainder = <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >,
{
    type Input = F0::Input;
    type ValidOutput = F1::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value).and_then(|v| {
            let (input, remainder) = v.split_take();
            F1::filter(state, input)
                .map(|v| <F1::ValidOutput as StateFilterInput<F1::ValidOutput>>::new(v, remainder))
        })
    }
}
impl<State, F0: StateFilter<State>, F1: StateFilter<State>, F2: StateFilter<State>>
    StateFilter<State> for (F0, F1, F2)
where
    F0::ValidOutput: StateFilterInput<F1::Input>,
    F1::ValidOutput: StateFilterInput<F2::Input>
        + StateFilterInput<
            F1::ValidOutput,
            Remainder = <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >,
    F2::ValidOutput: StateFilterInput<
            F2::ValidOutput,
            Remainder = <F1::ValidOutput as StateFilterInput<F2::Input>>::Remainder,
        >,
{
    type Input = F0::Input;
    type ValidOutput = F2::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value)
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| {
                    <F1::ValidOutput as StateFilterInput<F1::ValidOutput>>::new(v, remainder)
                })
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| {
                    <F2::ValidOutput as StateFilterInput<F2::ValidOutput>>::new(v, remainder)
                })
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
    F1::ValidOutput: StateFilterInput<F2::Input>
        + StateFilterInput<
            F1::ValidOutput,
            Remainder = <F0::ValidOutput as StateFilterInput<F1::Input>>::Remainder,
        >,
    F2::ValidOutput: StateFilterInput<F3::Input>
        + StateFilterInput<
            F2::ValidOutput,
            Remainder = <F1::ValidOutput as StateFilterInput<F2::Input>>::Remainder,
        >,
    F3::ValidOutput: StateFilterInput<
            F3::ValidOutput,
            Remainder = <F2::ValidOutput as StateFilterInput<F3::Input>>::Remainder,
        >,
{
    type Input = F0::Input;
    type ValidOutput = F3::ValidOutput;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        F0::filter(state, value)
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F1::filter(state, input).map(|v| {
                    <F1::ValidOutput as StateFilterInput<F1::ValidOutput>>::new(v, remainder)
                })
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F2::filter(state, input).map(|v| {
                    <F2::ValidOutput as StateFilterInput<F2::ValidOutput>>::new(v, remainder)
                })
            })
            .and_then(|v| {
                let (input, remainder) = v.split_take();
                F3::filter(state, input).map(|v| {
                    <F3::ValidOutput as StateFilterInput<F3::ValidOutput>>::new(v, remainder)
                })
            })
    }
}

pub trait StateFilterOutputCombination<T>: Sized {
    type Combined;
    fn combine(a: Self, b: T) -> Self::Combined;
}
impl<T, U> StateFilterOutputCombination<T> for U {
    type Combined = (U, T);
    fn combine(a: Self, b: T) -> Self::Combined {
        (a, b)
    }
}
impl<T0, T1, T2> StateFilterOutputCombination<(T0, T1)> for T2 {
    type Combined = (T0, T1, T2);
    fn combine(a: Self, (b, c): (T0, T1)) -> Self::Combined {
        (a, b, c)
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
impl<T, U> StateFilterInput<T> for U {
    type Remainder = U;
    fn new(input: T, remainder: Self::Remainder) -> Self {}
    fn split_take(self) -> (T, Self::Remainder) {}
}
impl<T, R> StateFilterInput<T> for (T, R) {
    type Remainder = R;
    fn new(input: T, remainder: Self::Remainder) -> Self {
        (input, remainder)
    }
    fn split_take(self) -> (T, Self::Remainder) {
        (self.0, self.1)
    }
}
macro_rules! impl_state_filter_input_single_remainder {
    (
        $($t:ident),* $(,)? |
        $($indexes:tt),* $(,)? |
        $remainder_index:tt
    ) => {
        impl<$($t,)* R> StateFilterInput<($($t),*)> for ($($t,)* R) {
            type Remainder = R;
            fn new(input: ($($t),*), remainder: Self::Remainder) -> Self {
                ($(input.$indexes,)* remainder)
            }
            fn split_take(self) -> (($($t),*), Self::Remainder) {
                (($(self.$indexes),*), self.$remainder_index)
            }
        }
    };
}

impl_state_filter_input_single_remainder!(T0, T1 | 0, 1 | 2);
impl_state_filter_input_single_remainder!(T0, T1, T2 | 0, 1, 2 | 3);
impl_state_filter_input_single_remainder!(T0, T1, T2, T3 | 0, 1, 2, 3 | 4);

macro_rules! impl_state_filter_input {
    (
        $($t:ident),* $(,)? |
        $($remainder:ident),* $(,)? |
        $($indexes:tt),* $(,)? |
        $($remainder_indexes:tt),* $(,)? |
        $($init_remainder_indexes:tt),* $(,)?
    ) => {
        impl<$($t,)* $($remainder,)*> StateFilterInput<($($t),*)> for ($($t,)* $($remainder,)*) {
            type Remainder = ($($remainder),*);
            fn new(input: ($($t),*), remainder: Self::Remainder) -> Self {
                ($(input.$indexes,)* $(remainder.$init_remainder_indexes,)*)
            }
            fn split_take(self) -> (($($t),*), Self::Remainder) {
                (($(
                    self.$indexes
                ),*), ($(
                    self.$remainder_indexes
                ),*))
            }
        }
    };
}

impl_state_filter_input!(T0, T2 | R0, R1 | 0, 1 | 2, 3 | 0, 1);
impl_state_filter_input!(T0, T2, T3 | R0, R1 | 0, 1, 2 | 3, 4 | 0, 1);
impl_state_filter_input!(T0, T1, T2, T3 | R0, R1 | 0, 1, 2, 3 | 4, 5 | 0, 1);
impl_state_filter_input!(T0, T1, T2 | R0, R1, R2 | 0, 1, 2 | 3, 4, 5 | 0, 1, 2);
impl_state_filter_input!(T0, T1 | R0, R1, R2, R3 | 0, 1 | 2, 3, 4, 5 | 0, 1, 2, 3);
