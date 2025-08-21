use card_game::{stack::priority::GetState, steps::Step};

use crate::{Game, steps::MainStep};

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
impl GetState<Game> for StartStep {
    fn state(&self) -> &Game {
        &self.game
    }
}
