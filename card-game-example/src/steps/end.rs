use card_game::{identifications::PlayerID, stack::priority::GetState, steps::Step};

use crate::{
    Game,
    steps::{GetStateMut, StartStep, StepMut},
};

pub struct EndStep {
    game: Game,
}

impl EndStep {
    pub(crate) fn new(game: Game) -> Self {
        EndStep { game }
    }
}

impl Step for EndStep {
    type State = Game;
    type NextStep = StartStep;
    fn next_step(mut self) -> Self::NextStep {
        self.game.player_manager.next_player_id();
        StartStep::new(self.game)
    }
}
impl StepMut for EndStep {
    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.game
    }
}
impl GetState<Game> for EndStep {
    fn state(&self) -> &Game {
        &self.game
    }
}
impl GetStateMut<Game> for EndStep {
    fn state_mut(&mut self) -> &mut Game {
        &mut self.game
    }
}
