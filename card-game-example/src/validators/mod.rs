use card_game::{
    cards::CardID, identifications::PlayerID, stack::priority::GetState, steps::Step,
    validation::Validator,
};

use crate::{
    Game,
    cards::{CardKind, monster::MonsterCard},
    filters::{CardIn, OfType},
    steps::GetStateMut,
    zones::hand::HandZone,
};

/*pub fn monster_card_in_hand_validator<State: GetState<Game>>(
    state: State,
    card_id: CardID,
) -> Option<Validator<State, (CardIn<HandZone>, OfType<MonsterCard>)>> {
    Validator::<State, (CardIn<HandZone>, OfType<MonsterCard>)>::try_new(state, |_state| card_id)
}*/
