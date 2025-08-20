#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayerID(usize);
impl PlayerID {
    pub(crate) fn next_player_id(&self, max_players: usize) -> Self {
        PlayerID((self.0 + 1) % max_players)
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
