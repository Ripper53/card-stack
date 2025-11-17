use std::collections::HashMap;

use crate::create_valid_identification;
use card_stack::NonEmptyInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerID(usize);
impl NonEmptyInput for PlayerID {}
impl PlayerID {
    fn next_player_id(&self, max_players: usize) -> Self {
        PlayerID((self.0 + 1) % max_players)
    }
}
impl std::fmt::Display for PlayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

use crate as card_game;
create_valid_identification!(ValidPlayerID, PlayerID, with_copy);
impl From<ValidPlayerID<ActivePlayer>> for ValidPlayerID<()> {
    fn from(valid_id: ValidPlayerID<ActivePlayer>) -> Self {
        ValidPlayerID(valid_id.0, std::marker::PhantomData::default())
    }
}
impl<F> ValidPlayerID<F> {
    pub(crate) fn new(player_id: PlayerID) -> Self {
        ValidPlayerID(player_id, std::marker::PhantomData::default())
    }
}
impl<F> ValidPlayerID<F> {
    pub fn try_new<P>(
        player_manager: &PlayerManager<P>,
        player_id: PlayerID,
    ) -> Result<Self, PlayerDoesNotExist> {
        if player_manager.players.contains_key(&player_id) {
            Ok(ValidPlayerID(
                player_id,
                std::marker::PhantomData::default(),
            ))
        } else {
            Err(PlayerDoesNotExist(player_id))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("player {0} does not exist")]
pub struct PlayerDoesNotExist(PlayerID);
#[derive(Debug)]
pub struct ActivePlayer;

pub struct PlayerManager<P> {
    current_player_id: PlayerID,
    pub(crate) players: HashMap<PlayerID, P>,
}

impl<P> PlayerManager<P> {
    /// `players`: must have at least one player
    pub fn new(players: HashMap<PlayerID, P>) -> Self {
        PlayerManager {
            current_player_id: PlayerID(0),
            players,
        }
    }
    pub fn active_player_id(&self) -> ValidPlayerID<ActivePlayer> {
        ValidPlayerID::new(self.current_player_id)
    }
    pub fn next_player_id(&mut self) -> ValidPlayerID<()> {
        self.current_player_id = self.current_player_id.next_player_id(self.players.len());
        ValidPlayerID::new(self.current_player_id)
    }
    pub fn player_count(&self) -> usize {
        self.players.len()
    }
    pub fn players(&self) -> impl Iterator<Item = ValidPlayerID<()>> {
        self.players
            .keys()
            .copied()
            .map(|player_id| ValidPlayerID::new(player_id))
    }
}

pub struct PlayerIDBuilder {
    next_player_id: usize,
}

impl PlayerIDBuilder {
    pub(crate) fn new() -> Self {
        PlayerIDBuilder { next_player_id: 0 }
    }
    /// Generates a unique player ID,
    /// call repeatedly for subsequent player IDs.
    pub fn generate_player_id(&mut self) -> PlayerID {
        let player_id = PlayerID(self.next_player_id);
        self.next_player_id += 1;
        player_id
    }
}
