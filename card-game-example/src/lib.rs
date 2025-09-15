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
    pub fn active_player_zones(&self) -> &Zones {
        let active_player_id = self.player_manager.active_player_id();
        active_player_id.get(|id| self.zone_manager.get_zone(id.player_id()))
    }
    pub fn active_player_zones_mut(&mut self) -> &mut Zones {
        let active_player_id = self.player_manager.active_player_id();
        active_player_id.get_mut(|id| self.zone_manager.get_zone_mut(id.player_id()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
