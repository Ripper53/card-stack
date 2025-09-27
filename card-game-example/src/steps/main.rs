use card_game::{
    cards::CardManager,
    commands::{Command, CommandManager},
    identifications::PlayerID,
    stack::priority::GetState,
    steps::Step,
    zones::ArrayZone,
};

use crate::{
    Game,
    steps::{EndStep, GetStateMut, StepMut},
    zones::hand::HandZone,
};

pub struct MainStep {
    pub(crate) game: Game,
    available_normal_summons: usize,
}
impl GetState<CardManager> for MainStep {
    fn state(&self) -> &CardManager {
        self.game.card_manager()
    }
}

impl MainStep {
    pub(crate) fn new(game: Game) -> Self {
        MainStep {
            game,
            available_normal_summons: 1,
        }
    }
    pub fn game(&self) -> &Game {
        &self.game
    }
    pub fn use_normal_summon(&mut self) -> bool {
        if self.available_normal_summons == 0 {
            false
        } else {
            self.available_normal_summons -= 1;
            true
        }
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
