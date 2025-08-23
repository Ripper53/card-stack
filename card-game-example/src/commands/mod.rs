use card_game::{
    SuperCommand,
    cards::CardID,
    commands::Command,
    zones::{FiniteZone, ValidCardID},
};

use crate::{steps::MainStep, zones::hand::HandZone};

mod play_card;

pub use play_card::*;

#[derive(SuperCommand)]
pub enum Commands<'a> {
    PlayCard(PlayCardCommand<'a>),
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
