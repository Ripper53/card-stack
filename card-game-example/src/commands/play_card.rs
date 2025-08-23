use card_game::{
    commands::Command,
    validation::{CardIn, ValidState},
    zones::{ArrayZone, FiniteZone, ValidCardID},
};

use crate::{
    steps::{MainStep, PlayCardTrait},
    zones::hand::HandZone,
};

pub struct PlayCardCommand<'a>(std::marker::PhantomData<&'a ()>);

impl<'a> Command for PlayCardCommand<'a> {
    type Data = ();
    type InState = ValidState<'a, MainStep, CardIn<HandZone>>;
    type OutState = MainStep;
    fn new((): ()) -> Self {
        PlayCardCommand(std::marker::PhantomData::default())
    }
    fn execute(&mut self, mut state: Self::InState) -> Self::OutState {
        state.play_card()
    }
    fn undo(self, state: Self::OutState) -> Self::InState {
        todo!()
    }
}
