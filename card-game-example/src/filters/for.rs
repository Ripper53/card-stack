use card_game::{
    identifications::{ActivePlayer, PlayerID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
};

use crate::{Game, filters::FilterInput};

pub struct For<F>(std::marker::PhantomData<F>);

impl<State: GetState<Game>> StateFilter<State, FilterInput<PlayerID>> for For<ActivePlayer> {
    type ValidOutput = ValidPlayerID<ActivePlayer>;
    fn filter(
        state: &State,
        FilterInput(player_id): FilterInput<PlayerID>,
    ) -> Option<Self::ValidOutput> {
        let active_player_id = state.state().player_manager().active_player_id();
        if active_player_id.id() == player_id {
            Some(active_player_id)
        } else {
            None
        }
    }
}
