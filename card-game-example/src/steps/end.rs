use card_game::{identifications::PlayerID, stack::priority::GetState, steps::Step};

use crate::{Game, steps::StartStep};

pub struct EndStep<'a> {
    game: Game<'a>,
}

impl<'a> EndStep<'a> {
    pub(crate) fn new(game: Game<'a>) -> Self {
        EndStep { game }
    }
}

impl<'a> Step for EndStep<'a> {
    type State = Game<'a>;
    type NextStep = StartStep<'a>;
    fn next_step(mut self) -> Self::NextStep {
        self.game.player_manager.next_player_id();
        StartStep::new(self.game)
    }
}

impl<'a> GetState<Game<'a>> for EndStep<'a> {
    fn state(&self) -> &Game<'a> {
        &self.game
    }
}
