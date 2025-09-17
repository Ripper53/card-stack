use card_game::{
    cards::CardID,
    stack::priority::GetState,
    steps::Step,
    validation::StateFilter,
    zones::{ValidCardID, Zone},
};

use crate::{Game, steps::MainStep};

pub struct CardIn<T>(std::marker::PhantomData<T>);

impl<'a, State: GetState<Game>, Z: Zone> StateFilter<State> for CardIn<Z> {
    type Input = CardID;
    type ValidOutput = ValidCardID<Self>;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        state.state();
        todo!();
        None
    }
}
