use std::collections::HashMap;

use crate::identifications::ValidCardID;
use crate::validation::{StateFilterInput, StateFilterInputConversion};
use crate::{create_valid_identification, validation::StateFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerID(usize);
impl StateFilterInput for PlayerID {}
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
create_valid_identification!(ValidPlayerID, PlayerID);
/*impl<F0, F1> StateFilterInputConversion<(ValidPlayerID<F0>, ValidCardID<F1>)>
    for (ValidPlayerID<F0>, ValidCardID<F1>)
{
    type Remainder = (T,);
    fn split_take(self) -> ((ValidPlayerID<F0>, ValidCardID<F1>), Self::Remainder) {
        ((self.0, self.1), (self.2,))
    }
}*/
impl<F0, F1, T> StateFilterInputConversion<(ValidPlayerID<F0>, ValidCardID<F1>)>
    for (ValidPlayerID<F0>, ValidCardID<F1>, T)
{
    type Remainder = (T,);
    fn split_take(self) -> ((ValidPlayerID<F0>, ValidCardID<F1>), Self::Remainder) {
        ((self.0, self.1), (self.2,))
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
