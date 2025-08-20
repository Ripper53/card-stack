use std::collections::HashMap;

use card_game::{CardGameBuilder, identifications::PlayerManager};
use card_game_example::{Game, player::Player};

#[derive(Default)]
pub struct GameBuilder<'a, const PLAYER_COUNT: usize>(std::marker::PhantomData<&'a ()>);
impl<'a, const PLAYER_COUNT: usize> CardGameBuilder for GameBuilder<'a, PLAYER_COUNT> {
    type GenerationData = ();
    type Game = Game<'a>;
    fn generate(
        mut player_id_builder: card_game::identifications::PlayerIDBuilder,
        card_builder: card_game::cards::CardBuilder,
        generation_data: Self::GenerationData,
    ) -> Self::Game {
        let players = {
            let mut players = HashMap::with_capacity(PLAYER_COUNT);
            for _ in 0..PLAYER_COUNT {
                let player_id = player_id_builder.generate_player_id();
                players.insert(player_id, Player::new(player_id));
            }
            players
        };
        Game::new(PlayerManager::new(players))
    }
}
