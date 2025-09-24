use card_game::{
    cards::CardID,
    identifications::{ActivePlayer, GetValidCardIDFromZone, PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    steps::Step,
    validation::{StateFilter, StateFilterInput},
    zones::Zone,
};

use crate::{
    Game,
    filters::{FilterInput, For},
    steps::MainStep,
    zones::GetZone,
};

pub struct CardIn<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, Z: GetZone> StateFilter<State, FilterInput<(PlayerID, CardID)>>
    for CardIn<Z>
{
    type ValidOutput = (ValidPlayerID<()>, ValidCardID<Self>);
    fn filter(
        state: &State,
        FilterInput((player_id, card_id)): FilterInput<(PlayerID, CardID)>,
    ) -> Option<Self::ValidOutput> {
        let state = state.state();
        let valid_player_id = ValidPlayerID::try_new(&state.player_manager, player_id)?;
        let valid_card_id = ValidCardID::try_new(card_id, Z::get_zone(state, &valid_player_id))?;
        Some((valid_player_id, valid_card_id))
    }
}

impl<State: GetState<Game>, Z: GetZone>
    StateFilter<State, FilterInput<(ValidPlayerID<()>, ValidCardID<Self>)>> for CardIn<Z>
{
    type ValidOutput = (ValidPlayerID<()>, ValidCardID<Self>);
    fn filter(
        state: &State,
        FilterInput((player_id, card_id)): FilterInput<(ValidPlayerID<()>, ValidCardID<Self>)>,
    ) -> Option<Self::ValidOutput> {
        Some((player_id, card_id))
    }
}
