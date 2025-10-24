use card_game::{
    identifications::{ActivePlayer, PlayerID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
};

use crate::{Game, filters::FilterInput};

pub struct For<F>(std::marker::PhantomData<F>);

impl<State: GetState<Game>> StateFilter<State, PlayerID> for For<ActivePlayer> {
    type ValidOutput = ValidPlayerID<ActivePlayer>;
    type Error = ActivePlayerError;
    fn filter(state: &State, player_id: PlayerID) -> Result<Self::ValidOutput, Self::Error> {
        let active_player_id = state.state().player_manager().active_player_id();
        if active_player_id.id() == player_id {
            Ok(active_player_id)
        } else {
            Err(ActivePlayerError(player_id))
        }
    }
}
impl<State: GetState<Game>> StateFilter<State, FilterInput<PlayerID>> for For<ActivePlayer> {
    type ValidOutput = FilterInput<ValidPlayerID<ActivePlayer>>;
    type Error = ActivePlayerError;
    fn filter(
        state: &State,
        FilterInput(player_id): FilterInput<PlayerID>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        Self::filter(state, player_id).map(FilterInput)
    }
}
#[derive(thiserror::Error, Debug)]
#[error("player {0} is not the active player")]
pub struct ActivePlayerError(PlayerID);
