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
    filters::{CardIn, FilterInput, OfType},
    player::Player,
    steps::{MainStep, StartStep},
    valid_actions::NormalSummonMonsterValidAction,
    zones::{SlotID, hand::HandZone},
};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let player_id = main.state().player_manager().active_player_id();
    let card_id = main
        .state()
        .zone_manager()
        .valid_zone(&player_id)
        .hand_zone()
        .cards()
        .next()
        .unwrap()
        .id();
    let player_id = player_id.id();
    let context = Validator::try_new(main, FilterInput((player_id, card_id, SlotID::new(0))))
        .expect("expected a card in hand");
    context.execute(NormalSummonMonsterValidAction::new(Position::Attack));
}
