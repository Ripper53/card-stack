use card_game::{stack::priority::GetState, steps::Step};

use crate::{Game, steps::MainStep};

pub struct StartStep<'a> {
    game: Game<'a>,
}

impl<'a> StartStep<'a> {
    pub fn new(game: Game<'a>) -> Self {
        StartStep { game }
    }
}

impl<'a> Step for StartStep<'a> {
    type State = Game<'a>;
    type NextStep = MainStep<'a>;
    fn next_step(self) -> Self::NextStep {
        MainStep::new(self.game)
    }
}
impl<'a> GetState<Game<'a>> for StartStep<'a> {
    fn state(&self) -> &Game<'a> {
        &self.game
    }
}
