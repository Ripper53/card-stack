use card_game::{stack::priority::GetState, steps::Step};

use crate::{Game, player::PlayerID, steps::MainStep};

pub struct StartStep<'a> {
    game: Game<'a>,
    active_player_id: PlayerID,
}

impl<'a> StartStep<'a> {
    pub(crate) fn new(game: Game<'a>, active_player_id: PlayerID) -> Self {
        StartStep {
            game,
            active_player_id,
        }
    }
}

impl<'a> Step for StartStep<'a> {
    type State = Game<'a>;
    type NextStep = MainStep<'a>;
    fn next_step(self) -> Self::NextStep {
        MainStep::new(self.game, self.active_player_id)
    }
}
impl<'a> GetState<Game<'a>> for StartStep<'a> {
    fn state(&self) -> &Game<'a> {
        &self.game
    }
}
