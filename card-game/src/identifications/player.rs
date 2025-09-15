use std::collections::HashMap;

use crate::validation::StateFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerID(usize);
impl StateFilter for PlayerID {
    type Value = Self;
    type Valid<'a> = ValidPlayerID<'a>;
}
impl PlayerID {
    fn next_player_id(&self, max_players: usize) -> Self {
        PlayerID((self.0 + 1) % max_players)
    }
}
pub struct ValidPlayerID<'a>(PlayerID, std::marker::PhantomData<(&'a (), *const ())>);
impl<'a> ValidPlayerID<'a> {
    pub(crate) fn new(player_id: PlayerID) -> Self {
        ValidPlayerID(player_id, std::marker::PhantomData::default())
    }
    pub fn player_id(&self) -> PlayerID {
        self.0
    }
}

pub struct PlayerManager<P> {
    current_player_id: PlayerID,
    pub(crate) players: HashMap<PlayerID, P>,
}

pub struct ActivePlayerID<'a>(PlayerID, std::marker::PhantomData<&'a ()>);
impl<'a> ActivePlayerID<'a> {
    fn new(player_id: PlayerID) -> Self {
        ActivePlayerID(player_id, std::marker::PhantomData::default())
    }
    pub fn player_id(&self) -> PlayerID {
        self.0
    }
    pub fn get<'b, T>(self, f: impl FnOnce(Self) -> Option<&'b T>) -> &'b T {
        f(self).unwrap()
    }
    pub fn get_mut<'b, T>(self, f: impl FnOnce(Self) -> Option<&'b mut T>) -> &'b mut T {
        f(self).unwrap()
    }
}
impl<'a> std::fmt::Debug for ActivePlayerID<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActivePlayerID({:?})", self.0)
    }
}
impl<'a> PartialEq for ActivePlayerID<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<'a> Eq for ActivePlayerID<'a> {}
impl<'a> std::hash::Hash for ActivePlayerID<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
impl<P> PlayerManager<P> {
    /// `players`: must have at least one player
    pub fn new(players: HashMap<PlayerID, P>) -> Self {
        PlayerManager {
            current_player_id: PlayerID(0),
            players,
        }
    }
    pub fn active_player_id(&self) -> ActivePlayerID<'_> {
        ActivePlayerID::new(self.current_player_id)
    }
    pub fn next_player_id(&mut self) -> ActivePlayerID<'_> {
        self.current_player_id = self.current_player_id.next_player_id(self.players.len());
        ActivePlayerID::new(self.current_player_id)
    }
    pub fn player_count(&self) -> usize {
        self.players.len()
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
