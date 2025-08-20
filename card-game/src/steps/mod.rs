use card_stack::priority::GetState;

pub trait Step: GetState<Self::State> {
    type State;
    type NextStep: Step<State = Self::State>;
    fn next_step(self) -> Self::NextStep;
}
