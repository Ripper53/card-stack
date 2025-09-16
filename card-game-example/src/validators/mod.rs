use card_game::{
    cards::CardID,
    stack::priority::GetState,
    steps::Step,
    validation::{Validator, ZoneCardValidationError, filters::CardIn},
};

use crate::{
    Game,
    cards::{CardKind, monster::MonsterCard},
    filters::OfType,
    steps::GetStateMut,
    zones::hand::HandZone,
};

pub fn monster_card_in_hand_validator<State: GetState<Game>>(
    state: State,
    card_id: CardID,
) -> Result<Validator<State, CardIn<(HandZone, OfType<MonsterCard>)>>, ZoneCardValidationError> {
    Validator::<State, CardIn<(HandZone, OfType<MonsterCard>)>>::try_new(
        state,
        |state| {
            &<State as GetState<Game>>::state(&state)
                .active_player_zones()
                .hand_zone
        },
        |zone_context| {
            if let Some((valid_card_id, card)) = zone_context.get_zone_card(card_id)
                && let CardKind::Monster(_) = card.kind()
            {
                Some(valid_card_id)
            } else {
                None
            }
        },
    )
}
