use card_game::{
    cards::CardID,
    identifications::{GetValidCardIDFromZone, PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    steps::Step,
    validation::StateFilter,
    zones::Zone,
};

use crate::{Game, steps::MainStep, zones::GetZone};

pub struct CardIn<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, Z: GetZone> StateFilter<State, (PlayerID, CardID)> for CardIn<Z> {
    type ValidOutput = (ValidPlayerID<()>, ValidCardID<Self>);
    fn filter(
        state: &State,
        (player_id, card_id): (PlayerID, CardID),
    ) -> Option<Self::ValidOutput> {
        let state = state.state();
        let valid_player_id = ValidPlayerID::try_new(&state.player_manager, player_id)?;
        let valid_card_id = ValidCardID::try_new(card_id, Z::get_zone(state, &valid_player_id))?;
        Some((valid_player_id, valid_card_id))
    }
}

impl<State: GetState<Game>, Z: GetZone, F> StateFilter<State, (ValidPlayerID<F>, CardID)>
    for CardIn<Z>
{
    type ValidOutput = (ValidPlayerID<F>, ValidCardID<Self>);
    fn filter(
        state: &State,
        (valid_player_id, card_id): (ValidPlayerID<F>, CardID),
    ) -> Option<Self::ValidOutput> {
        let state = state.state();
        let valid_card_id = ValidCardID::try_new(card_id, Z::get_zone(state, &valid_player_id))?;
        Some((valid_player_id, valid_card_id))
    }
}
