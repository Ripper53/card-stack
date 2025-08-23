use card_game::{
    commands::{Command, CommandManager},
    identifications::PlayerID,
    stack::priority::GetState,
    steps::Step,
    validation::{CardIn, ValidState},
    zones::{ArrayZone, ValidCardID},
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
    pub fn work_as_mut(&mut self) {
        // PRETEND MUTATE
    }
}
impl<'a> PlayCardTrait for ValidState<'a, MainStep, CardIn<HandZone>> {
    fn play_card(mut self) -> MainStep {
        let (mut main_step, valid_player_id, valid_card_id) = self.take_all();
        let card = main_step
            .game
            .active_player_zones_mut()
            .hand_zone
            .remove_card(valid_card_id);
        main_step
    }
}
pub trait PlayCardTrait {
    fn play_card(self) -> MainStep;
}

pub struct PlayCard<'a> {
    zone_card_id: ValidCardID<'a, HandZone>,
}
