use card_game::{
    identifications::{ActivePlayer, PlayerID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
};

use crate::Game;

pub struct For<F>(std::marker::PhantomData<F>);

impl<State: GetState<Game>> StateFilter<State, PlayerID> for For<ActivePlayer> {
    type ValidOutput = ValidPlayerID<ActivePlayer>;
    fn filter(state: &State, player_id: PlayerID) -> Option<Self::ValidOutput> {
        let active_player_id = state.state().player_manager().active_player_id();
        if active_player_id.id() == player_id {
            Some(active_player_id)
        } else {
            None
        }
    }
}

impl<State: GetState<Game>> StateFilter<State, ValidPlayerID<()>> for For<ActivePlayer> {
    type ValidOutput = ValidPlayerID<ActivePlayer>;
    fn filter(state: &State, valid_player_id: ValidPlayerID<()>) -> Option<Self::ValidOutput> {
        let active_player_id = state.state().player_manager().active_player_id();
        if active_player_id.id() == valid_player_id.id() {
            Some(active_player_id)
        } else {
            None
        }
    }
}

impl<State: GetState<Game>, I> StateFilter<State, (PlayerID, I)> for For<ActivePlayer> {
    type ValidOutput = (ValidPlayerID<ActivePlayer>, I);
    fn filter(state: &State, (player_id, input): (PlayerID, I)) -> Option<Self::ValidOutput> {
        let active_player_id = state.state().player_manager().active_player_id();
        if active_player_id.id() == player_id {
            Some((active_player_id, input))
        } else {
            None
        }
    }
}

impl<State: GetState<Game>, I> StateFilter<State, (ValidPlayerID<()>, I)> for For<ActivePlayer> {
    type ValidOutput = (ValidPlayerID<ActivePlayer>, I);
    fn filter(
        state: &State,
        (valid_player_id, input): (ValidPlayerID<()>, I),
    ) -> Option<Self::ValidOutput> {
        let active_player_id = state.state().player_manager().active_player_id();
        if active_player_id.id() == valid_player_id.id() {
            Some((active_player_id, input))
        } else {
            None
        }
    }
}
