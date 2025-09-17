use std::collections::HashMap;

use card_game::{
    identifications::{PlayerID, PlayerManager},
    zones::ZoneManager,
};

use crate::{player::Player, steps::StartStep, zones::Zones};

pub mod cards;
pub mod filters;
pub mod player;
pub mod steps;
pub mod valid_actions;
pub mod validators;
pub mod zones;

pub struct Game {
    player_manager: PlayerManager<Player>,
    zone_manager: ZoneManager<Zones>,
}

impl Game {
    pub fn start_step(player_manager: PlayerManager<Player>) -> StartStep {
        StartStep::new(Self::new(player_manager))
    }
    pub fn new(player_manager: PlayerManager<Player>) -> Self {
        Game {
            zone_manager: ZoneManager::new(&player_manager),
            player_manager,
        }
    }
    pub fn player_manager(&self) -> &PlayerManager<Player> {
        &self.player_manager
    }
    pub fn zone_manager(&self) -> &ZoneManager<Zones> {
        &self.zone_manager
    }
    pub fn active_player_zones(&self) -> &Zones {
        let active_player_id = self.player_manager.active_player_id();
        self.zone_manager.get_zone(active_player_id.id()).unwrap()
    }
    pub fn active_player_zones_mut(&mut self) -> &mut Zones {
        let active_player_id = self.player_manager.active_player_id();
        self.zone_manager
            .get_zone_mut(active_player_id.id())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
