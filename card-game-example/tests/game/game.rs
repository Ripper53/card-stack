use std::collections::HashMap;

use card_game::{
    CardGameBuilder,
    commands::CommandManager,
    stack::priority::GetState,
    steps::Step,
    zones::{ValidZoneCardContext, Zone, ZoneContext},
};
use card_game_example::{Game, commands::Commands, player::Player, steps::StartStep};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let mut command_manager = CommandManager::<Commands>::new();
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let context = ValidZoneCardContext::new(
        main,
        |main| main.state().active_player_zones().hand_zone(),
        |hand_zone| {
            let hand_card_id = hand_zone.get_zone_card_id_from_index(0).unwrap();
            hand_card_id
        },
    );
    context.execute(|main, hand_card_id| {
        let mut main = main.play_card(&mut command_manager, hand_card_id);
        //main.play_card(&mut command_manager, hand_card_id);
    });
}
