use card_game::{stack::priority::GetState, steps::Step};

use crate::{Game, player::PlayerID, steps::StartStep};

pub struct EndStep<'a> {
    game: Game<'a>,
    active_player_id: PlayerID,
}

impl<'a> EndStep<'a> {
    pub(crate) fn new(game: Game<'a>, active_player_id: PlayerID) -> Self {
        EndStep {
            game,
            active_player_id,
        }
    }
}

impl<'a> Step for EndStep<'a> {
    type State = Game<'a>;
    type NextStep = StartStep<'a>;
    fn next_step(self) -> Self::NextStep {
        let player_count = self.game.players.len();
        StartStep::new(
            self.game,
            self.active_player_id.next_player_id(player_count),
        )
    }
}

impl<'a> GetState<Game<'a>> for EndStep<'a> {
    fn state(&self) -> &Game<'a> {
        &self.game
    }
}
