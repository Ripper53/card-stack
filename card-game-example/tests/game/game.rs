use std::collections::HashMap;

use card_game::{
    CardGameBuilder,
    commands::CommandManager,
    stack::priority::GetState,
    steps::Step,
    validation::Validator,
    zones::{Zone, ZoneContext},
};
use card_game_example::{
    Game,
    cards::monster::{MonsterCard, Position},
    filters::{CardIn, OfType},
    player::Player,
    steps::{MainStep, StartStep},
    valid_actions::PlayMonsterCardValidAction,
    zones::hand::HandZone,
};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let player_id = main.state().player_manager().active_player_id().id();
    let card_id = main
        .state()
        .active_player_zones()
        .hand_zone()
        .cards()
        .next()
        .unwrap()
        .id();
    let context = Validator::try_new(main, move |_state| (player_id, card_id))
        .expect("expected a card in hand");
    context.execute(PlayMonsterCardValidAction::new(0, Position::Attack));
}
