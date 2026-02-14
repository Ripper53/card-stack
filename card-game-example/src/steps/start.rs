use card_game::{stack::priority::GetState, steps::Step};

use crate::{
    Game,
    steps::{GetStateMut, MainStep, StepMut},
};

pub struct StartStep {
    game: Game,
}

impl StartStep {
    pub fn new(game: Game) -> Self {
        StartStep { game }
    }
}

impl Step for StartStep {
    type State = Game;
    type NextStep = MainStep;
    fn next_step(self) -> Self::NextStep {
        MainStep::new(self.game)
    }
}
impl StepMut for StartStep {
    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.game
    }
}
impl GetState<Game> for StartStep {
    fn state(&self) -> &Game {
        &self.game
    }
}
impl GetStateMut<Game> for StartStep {
    fn state_mut(&mut self) -> &mut Game {
        &mut self.game
    }
}
