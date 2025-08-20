use card_game::identifications::PlayerID;

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
