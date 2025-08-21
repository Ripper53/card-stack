use card_game::{
    commands::{Command, CommandManager},
    identifications::PlayerID,
    stack::priority::GetState,
    steps::Step,
    zones::ZoneCardID,
};

use crate::{
    Game,
    commands::{Commands, PlayCardCommand},
    steps::EndStep,
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

impl GetState<Game> for MainStep {
    fn state(&self) -> &Game {
        &self.game
    }
}

impl MainStep {
    pub fn play_card<'a>(
        self,
        command_manager: &mut CommandManager<Commands<'a>>,
        hand_card_id: ZoneCardID<'a, HandZone>,
    ) -> <PlayCardCommand<'a> as Command>::OutState {
        command_manager.execute::<PlayCardCommand<'a>>(hand_card_id, self)
    }
}
