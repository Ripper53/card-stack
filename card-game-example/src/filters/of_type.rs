use card_game::{
    identifications::ValidPlayerID,
    stack::priority::GetState,
    validation::StateFilter,
    zones::{ValidCardID, Zone},
};

use crate::{
    Game,
    cards::{CardKind, monster::MonsterCard},
    filters::CardIn,
    steps::MainStep,
    zones::hand::HandZone,
};

pub struct OfType<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, F> StateFilter<State, (ValidPlayerID<F>, ValidCardID<CardIn<HandZone>>)>
    for OfType<MonsterCard>
{
    type ValidOutput = (ValidPlayerID<F>, ValidCardID<(CardIn<HandZone>, Self)>);
    fn filter(
        state: &State,
        (valid_player_id, valid_card_id): (ValidPlayerID<F>, ValidCardID<CardIn<HandZone>>),
    ) -> Option<Self::ValidOutput> {
        let card = state
            .state()
            .zone_manager()
            .get_valid_zone(&valid_player_id)
            .hand_zone()
            .get_valid_card(&valid_card_id);
        if matches!(card.kind(), CardKind::Monster(_)) {
            Some((valid_player_id, valid_card_id.unchecked_replace_filter()))
        } else {
            None
        }
    }
}
