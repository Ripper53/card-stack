pub use card_stack as stack;
use card_stack::priority::GetState;

pub mod abilities;

pub trait Step: GetState<Self::State> {
    type State;
    type NextStep: Step<State = Self::State>;
    fn next_step(self) -> Self::NextStep;
}
