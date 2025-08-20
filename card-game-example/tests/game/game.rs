use std::collections::HashMap;

use card_game::{CardGameBuilder, steps::Step};
use card_game_example::{Game, player::Player, steps::StartStep};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let mut step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
}
