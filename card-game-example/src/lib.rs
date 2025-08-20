use std::collections::HashMap;

use crate::{
    player::{Player, PlayerID},
    steps::StartStep,
    zones::Zones,
};

pub mod cards;
pub mod commands;
pub mod player;
pub mod steps;
pub mod zones;

pub struct Game<'a> {
    players: HashMap<PlayerID, Player>,
    zones: HashMap<PlayerID, Zones<'a>>,
}

impl<'a> Game<'a> {
    pub fn start_step(players: HashMap<PlayerID, Player>) -> StartStep<'a> {
        StartStep::new(Self::new(players), PlayerID::new(0))
    }
    pub fn new(players: HashMap<PlayerID, Player>) -> Self {
        let mut zones = HashMap::with_capacity(players.len());
        for player_id in players.keys().copied() {
            zones.insert(player_id, Zones::new());
        }
        Game { players, zones }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game() {
        let start_step = Game::start_step(two_players());
    }

    fn two_players() -> HashMap<PlayerID, Player> {
        let mut players = HashMap::with_capacity(2);
        players.insert(PlayerID::new(0), Player::new(PlayerID::new(0)));
        players.insert(PlayerID::new(1), Player::new(PlayerID::new(1)));
        players
    }
}
