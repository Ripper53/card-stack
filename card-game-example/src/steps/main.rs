use card_game::{
    cards::{CardActions, CardManager},
    commands::{Command, CommandManager},
    events::GetEventManager,
    identifications::PlayerID,
    stack::priority::GetState,
    steps::Step,
    zones::ArrayZone,
};

use crate::{
    Game,
    events::{
        EventManager,
        summon::{NormalSummoned, SpecialSummoned},
    },
    steps::{EndStep, GetStateMut, StepMut},
    zones::hand::HandZone,
};

pub struct MainStep {
    pub(crate) game: Game,
    available_normal_summons: usize,
}
impl From<MainStep> for Game {
    fn from(main_step: MainStep) -> Self {
        main_step.game
    }
}
impl GetState<MainStep> for MainStep {
    fn state(&self) -> &MainStep {
        &self
    }
}
impl GetState<CardActions> for MainStep {
    fn state(&self) -> &CardActions {
        self.game.card_manager().card_actions()
    }
}
impl GetState<CardManager<EventManager>> for MainStep {
    fn state(&self) -> &CardManager<EventManager> {
        self.game.card_manager()
    }
}
impl GetEventManager<MainStep, NormalSummoned> for MainStep {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, NormalSummoned, Self::Output> {
        self.game.event_manager().event_manager()
    }
}
impl GetEventManager<MainStep, SpecialSummoned> for MainStep {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, SpecialSummoned, Self::Output> {
        self.game.event_manager().event_manager()
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
