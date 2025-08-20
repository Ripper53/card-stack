use std::collections::HashMap;

use card_game_example::{
    Game,
    player::{Player, PlayerID},
};

pub fn new_game<'a>() -> Game<'a> {
    let mut players = HashMap::with_capacity(2);
    players.insert(PlayerID::new(0), Player::new(PlayerID::new(0)));
    players.insert(PlayerID::new(1), Player::new(PlayerID::new(1)));
    Game::new(players)
}
