use std::collections::HashMap;

use card_game::{
    CardGameBuilder,
    commands::CommandManager,
    events::{EventActionResolution, TriggeredEventResolution},
    identifications::SourceCardID,
    stack::priority::GetState,
    steps::Step,
    zones::{Zone, ZoneContext},
};
use card_game_example::{
    Game,
    cards::{
        CardName,
        monster::{MonsterCard, Position},
        specifics::NeoKaiserSeaHorseSpecialSummon,
    },
    events::summon::SpecialSummoned,
    filters::{CardIn, OfType},
    player::Player,
    steps::{MainStep, StartStep},
    valid_actions::{
        NormalSummon, NormalSummonInput, PassiveGiveAttack, SpecialSummon, TributeSummon,
    },
    zones::{SlotID, hand::HandZone},
};
use state_validation::Validator;

use crate::utilities::GameBuilder;

#[test]
fn normal_summon() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let player_id = main.game().player_manager().active_player_id();
    assert_eq!(
        main.game()
            .zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        4,
    );
    assert_eq!(
        main.game()
            .zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        0,
    );
    let card = main
        .game()
        .zone_manager()
        .valid_zone(&player_id)
        .hand_zone()
        .cards()
        .skip(2)
        .next()
        .unwrap();
    let card_id = card.id();
    let player_id = player_id.id();

    let context = Validator::try_new(
        main,
        NormalSummonInput {
            player_id,
            card_id,
            slot_id: SlotID::new(0),
        },
    )
    .expect("expected a card in hand that can be normal summoned");
    let normal_summon_event = context.execute(NormalSummon::new(Position::Attack));

    let game = normal_summon_event.state().game();
    let player_id = game.player_manager().active_player_id();
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        3,
    );
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        1,
    );
}

#[test]
fn special_summon() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let player_id = main.game().player_manager().active_player_id();
    assert_eq!(
        main.game()
            .zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        4,
    );
    assert_eq!(
        main.game()
            .zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        0,
    );
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

    let game = event.state().game();
    let player_id = game.player_manager().active_player_id();
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        3,
    );
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        1,
    );
    //let r = event.consume::<>(todo!());
    //context.execute(NormalSummonMonster::new(Position::Attack));
}

#[test]
fn card_event_system() {
    let step = StartStep::new(GameBuilder::<'_, 2>::new(()));
    let mut main = step.next_step();
    let player_id = main.game().player_manager().active_player_id();
    assert_eq!(
        main.game()
            .zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        4,
    );
    assert_eq!(
        main.game()
            .zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        0,
    );
    let card = main
        .game()
        .zone_manager()
        .valid_zone(&player_id)
        .hand_zone()
        .cards()
        .skip(3)
        .next()
        .unwrap();
    let card_id = card.id();
    let player_id = player_id.id();

    let context = Validator::try_new(
        main,
        NormalSummonInput {
            player_id,
            card_id,
            slot_id: SlotID::new(0),
        },
    )
    .expect("expected a card in hand that can be normal summoned");
    let normal_summon_event = context.execute(NormalSummon::new(Position::Attack));
    let events = normal_summon_event.collect();

    let main_step = match events {
        TriggeredEventResolution::None(state) => state,
        TriggeredEventResolution::Action(_) => unreachable!(),
        TriggeredEventResolution::SimultaneousActions(actions) => {
            unreachable!(
                "Simultaneous Actions Count: {}",
                actions.simultaneous_action_count()
            );
        }
    };

    let game = main_step.game();
    let player_id = game.player_manager().active_player_id();
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        3,
    );
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        1,
    );

    let card = main_step
        .game()
        .zone_manager()
        .valid_zone(&player_id)
        .hand_zone()
        .cards()
        .skip(2)
        .next()
        .unwrap();
    let card_id = card.id();
    let player_id = player_id.id();
    let context = Validator::try_new(
        main_step,
        NormalSummonInput {
            player_id,
            card_id,
            slot_id: SlotID::new(1),
        },
    )
    .expect("expected a card in hand that can be normal summoned");
    let normal_summon_event = context.execute(NormalSummon::new(Position::Attack));

    let main_step = match normal_summon_event.collect() {
        TriggeredEventResolution::Action(action) => match action.resolve() {
            EventActionResolution::Resolved(main_step) => main_step,
            EventActionResolution::Fizzled { .. } => unreachable!(),
        },
        TriggeredEventResolution::SimultaneousActions(_) => unreachable!(),
        TriggeredEventResolution::None(_) => unreachable!(),
    };
    let game = main_step.game();
    let player_id = game.player_manager().active_player_id();
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .hand_zone()
            .filled_count(),
        2,
    );
    assert_eq!(
        game.zone_manager()
            .valid_zone(&player_id)
            .monster_zone()
            .filled_count(),
        2,
    );
}
