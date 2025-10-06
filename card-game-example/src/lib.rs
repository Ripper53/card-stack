use std::collections::HashMap;

use card_game::{
    cards::CardManager,
    identifications::{PlayerID, PlayerManager},
    stack::priority::GetState,
    zones::ZoneManager,
};

use crate::{
    events::EventManager,
    player::Player,
    steps::{GetStateMut, StartStep},
    zones::Zones,
};

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
    card_manager: CardManager<EventManager>,
}
impl GetState<Game> for Game {
    fn state(&self) -> &Game {
        self
    }
}
impl GetStateMut<Game> for Game {
    fn state_mut(&mut self) -> &mut Game {
        self
    }
}

impl Game {
    pub fn start_step(
        player_manager: PlayerManager<Player>,
        card_manager: CardManager<EventManager>,
    ) -> StartStep {
        StartStep::new(Self::new(player_manager, card_manager))
    }
    pub fn new(
        player_manager: PlayerManager<Player>,
        card_manager: CardManager<EventManager>,
    ) -> Self {
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
    pub fn card_manager(&self) -> &CardManager<EventManager> {
        &self.card_manager
    }
    pub fn card_manager_mut(&mut self) -> &mut CardManager<EventManager> {
        &mut self.card_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
