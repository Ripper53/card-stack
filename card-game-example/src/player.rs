pub struct Player {
    id: PlayerID,
}

impl Player {
    pub fn new(id: PlayerID) -> Self {
        Player { id }
    }
    pub fn id(&self) -> PlayerID {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerID(usize);
impl PlayerID {
    pub const fn new(id: usize) -> Self {
        PlayerID(id)
    }
    pub fn next_player_id(&self, max_players: usize) -> Self {
        PlayerID::new((self.0 + 1) % max_players)
    }
}
