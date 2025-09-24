use std::collections::HashMap;

use card_game::{
    CardGameBuilder,
    cards::{Card, CardManager},
    identifications::{PlayerID, PlayerManager},
    zones::FiniteZone,
};
use card_game_example::{
    Game,
    cards::{CardKind, monster::MonsterCard, specifics::BlueEyesWhiteDestinyConstructedDeck},
    player::Player,
};

#[derive(Default)]
pub struct GameBuilder<'a, const PLAYER_COUNT: usize>(std::marker::PhantomData<&'a ()>);
impl<'a, const PLAYER_COUNT: usize> CardGameBuilder for GameBuilder<'a, PLAYER_COUNT> {
    type GenerationData = ();
    type Game = Game;
    fn generate(
        mut player_id_builder: card_game::identifications::PlayerIDBuilder,
        mut card_manager: CardManager,
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
        let card_builder = card_manager.builder();
        let mut game = Game::new(PlayerManager::new(players));
        for valid_player_id in game.player_manager().players().collect::<Vec<_>>() {
            let card = card_builder.blue_eyes_white_dragon();
            game.zone_manager_mut()
                .valid_zone_mut(valid_player_id)
                .hand_zone_mut()
                .add_card(card.into_kind())
                .unwrap();
        }
        game
    }
}
