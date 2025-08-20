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

pub struct MainStep<'a> {
    pub(crate) game: Game<'a>,
}

impl<'a> MainStep<'a> {
    pub(crate) fn new(game: Game<'a>) -> Self {
        MainStep { game }
    }
}

impl<'a> Step for MainStep<'a> {
    type State = Game<'a>;
    type NextStep = EndStep<'a>;
    fn next_step(self) -> Self::NextStep {
        EndStep::new(self.game)
    }
}

impl<'a> GetState<Game<'a>> for MainStep<'a> {
    fn state(&self) -> &Game<'a> {
        &self.game
    }
}

impl<'a> MainStep<'a> {
    pub fn play_card(
        self,
        command_manager: &mut CommandManager<Commands<'a>>,
        hand_card_id: ZoneCardID<'a, HandZone>,
    ) -> <PlayCardCommand<'a> as Command>::OutState {
        command_manager.execute::<PlayCardCommand<'a>>(hand_card_id, self)
    }
}
