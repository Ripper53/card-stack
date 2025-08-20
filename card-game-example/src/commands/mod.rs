use card_game::{SuperCommand, cards::CardID, commands::Command, zones::ZoneCardID};

use crate::{steps::MainStep, zones::hand::HandZone};

#[derive(SuperCommand)]
pub enum Commands<'a> {
    PlayCard(PlayCardCommand<'a>),
}

pub struct PlayCardCommand<'a>(ZoneCardID<'a, HandZone>);

impl<'a> Command for PlayCardCommand<'a> {
    type Data = ZoneCardID<'a, HandZone>;
    type InState = MainStep<'a>;
    type OutState = MainStep<'a>;
    fn new(card_id: Self::Data) -> Self {
        PlayCardCommand(card_id)
    }
    fn execute(&mut self, state: Self::InState) -> Self::OutState {
        state
    }
    fn undo(self, state: Self::OutState) -> Self::InState {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use card_game::commands::CommandManager;

    use super::*;
    #[test]
    fn super_command() {
        let mut command_manager = CommandManager::<Commands>::new();
        //command_manager.execute::<PlayCardCommand>(CardID, state)
    }
}
