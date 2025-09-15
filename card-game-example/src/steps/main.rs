use card_game::{
    commands::{Command, CommandManager},
    identifications::PlayerID,
    stack::priority::GetState,
    steps::Step,
    validation::{ValidState, filters::CardIn},
    zones::{ArrayZone, ValidCardID},
};

use crate::{
    Game,
    steps::{EndStep, GetStateMut, StepMut},
    zones::hand::HandZone,
};

pub struct MainStep {
    pub(crate) game: Game,
}

impl MainStep {
    pub(crate) fn new(game: Game) -> Self {
        MainStep { game }
    }
}

impl Step for MainStep {
    type State = Game;
    type NextStep = EndStep;
    fn next_step(self) -> Self::NextStep {
        EndStep::new(self.game)
    }
}
impl StepMut for MainStep {
    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.game
    }
}
impl GetState<Game> for MainStep {
    fn state(&self) -> &Game {
        &self.game
    }
}
impl GetStateMut<Game> for MainStep {
    fn state_mut(&mut self) -> &mut Game {
        &mut self.game
    }
}
