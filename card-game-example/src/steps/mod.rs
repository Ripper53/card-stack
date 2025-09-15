use card_game::{stack::priority::GetState, steps::Step};

mod end;
mod main;
mod start;

pub use end::*;
pub use main::*;
pub use start::*;

pub(crate) trait StepMut: Step {
    fn state_mut(&mut self) -> &mut Self::State;
}
pub(crate) trait GetStateMut<State>: GetState<State> {
    fn state_mut(&mut self) -> &mut State;
}
