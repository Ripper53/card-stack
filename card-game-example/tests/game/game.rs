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
    commands::{Commands, PlayCardCommand},
    player::Player,
    steps::StartStep,
};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let mut command_manager = CommandManager::<Commands>::new();
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let context = Validator::new(
        main,
        |main| main.state().active_player_zones().hand_zone(),
        |hand_zone| hand_zone.get_zone_card_id_from_index(0),
    )
    .expect("expected a card in hand");
    context.execute(|main| {
        command_manager.execute::<PlayCardCommand>((), main);
    });
}
