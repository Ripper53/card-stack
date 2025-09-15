use std::collections::HashMap;

use card_game::{
    CardGameBuilder,
    commands::CommandManager,
    stack::priority::GetState,
    steps::Step,
    validation::{Validator, filters::CardIn},
    zones::{Zone, ZoneContext},
};
use card_game_example::{
    Game,
    player::Player,
    steps::{MainStep, StartStep},
    valid_actions::PlayCardValidAction,
    zones::hand::HandZone,
};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let context = Validator::<MainStep, CardIn<HandZone>>::try_new(
        main,
        |main| main.state().active_player_zones().hand_zone(),
        |hand_zone| hand_zone.get_zone_card_id_from_index(0),
    )
    .expect("expected a card in hand");
    context.execute(|main_step| {
        let main_step = main_step.execute(PlayCardValidAction);
    });
}
