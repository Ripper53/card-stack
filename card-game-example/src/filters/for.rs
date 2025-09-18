use card_game::{
    identifications::{ActivePlayer, PlayerID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
};

use crate::Game;

pub struct For<F>(std::marker::PhantomData<F>);

impl<State: GetState<Game>> StateFilter<State> for For<ActivePlayer> {
    type Input = ValidPlayerID<()>;
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
