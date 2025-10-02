use std::collections::HashMap;

use card_game::{
    CardGameBuilder,
    commands::CommandManager,
    identifications::SourceCardID,
    stack::priority::GetState,
    steps::Step,
    validation::Validator,
    zones::{Zone, ZoneContext},
};
use card_game_example::{
    Game,
    cards::{
        CardName,
        monster::{MonsterCard, Position},
        specifics::NeoKaiserSeaHorseSpecialSummon,
    },
    filters::{CardIn, FilterInput, OfType},
    player::Player,
    steps::{MainStep, StartStep},
    valid_actions::{NormalSummonMonster, SpecialSummon, TributeSummon},
    zones::{SlotID, hand::HandZone},
};

use crate::utilities::GameBuilder;

#[test]
fn game() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let player_id = main.game().player_manager().active_player_id();
    let card = main
        .game()
        .zone_manager()
        .valid_zone(&player_id)
        .hand_zone()
        .cards()
        .next()
        .unwrap();
    let card_id = card.id();
    let player_id = player_id.id();
    /*let context = Validator::try_new(main, FilterInput((player_id, card_id, SlotID::new(0))))
        .expect("expected a card in hand");
    let main = context.execute(NormalSummonMonster::new(Position::Attack));*/
    let context = Validator::try_new(
        main,
        NeoKaiserSeaHorseSpecialSummon {
            player_id,
            source_card_id: SourceCardID(card_id),
            slot_id: SlotID::new(0),
        },
    )
    .unwrap();
    let event = context.execute(SpecialSummon::new(Position::Attack));
    //let r = event.consume::<>(todo!());
    //context.execute(NormalSummonMonster::new(Position::Attack));
}
