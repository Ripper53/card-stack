use card_game::{stack::priority::GetState, steps::Step};

use crate::{Game, player::PlayerID, steps::EndStep};

pub struct MainStep<'a> {
    game: Game<'a>,
    active_player_id: PlayerID,
}

impl<'a> MainStep<'a> {
    pub(crate) fn new(game: Game<'a>, active_player_id: PlayerID) -> Self {
        MainStep {
            game,
            active_player_id,
        }
    }
}

impl<'a> Step for MainStep<'a> {
    type State = Game<'a>;
    type NextStep = EndStep<'a>;
    fn next_step(self) -> Self::NextStep {
        EndStep::new(self.game, self.active_player_id)
    }
}

impl<'a> GetState<Game<'a>> for MainStep<'a> {
    fn state(&self) -> &Game<'a> {
        &self.game
    }
}
