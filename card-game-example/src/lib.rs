use std::collections::HashMap;

use card_game::{
    cards::CardManager,
    identifications::{PlayerID, PlayerManager},
    zones::ZoneManager,
};

use crate::{player::Player, steps::StartStep, zones::Zones};

pub mod cards;
pub mod events;
pub mod filters;
pub mod identifications;
pub mod player;
pub mod steps;
pub mod valid_actions;
pub mod validators;
pub mod zones;

pub struct Game {
    player_manager: PlayerManager<Player>,
    zone_manager: ZoneManager<Zones>,
    card_manager: CardManager,
}

impl Game {
    pub fn start_step(
        player_manager: PlayerManager<Player>,
        card_manager: CardManager,
    ) -> StartStep {
        StartStep::new(Self::new(player_manager, card_manager))
    }
    pub fn new(player_manager: PlayerManager<Player>, card_manager: CardManager) -> Self {
        Game {
            zone_manager: ZoneManager::new(&player_manager),
            player_manager,
            card_manager,
        }
    }
    pub fn player_manager(&self) -> &PlayerManager<Player> {
        &self.player_manager
    }
    pub fn zone_manager(&self) -> &ZoneManager<Zones> {
        &self.zone_manager
    }
    pub fn zone_manager_mut(&mut self) -> &mut ZoneManager<Zones> {
        &mut self.zone_manager
    }
    pub fn card_manager(&self) -> &CardManager {
        &self.card_manager
    }
    pub fn card_manager_mut(&mut self) -> &mut CardManager {
        &mut self.card_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
