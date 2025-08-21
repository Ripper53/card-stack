mod manager;

pub use manager::*;

pub struct Card<Kind> {
    id: CardID,
    kind: Kind,
}

impl<Kind> Card<Kind> {
    pub fn id(&self) -> CardID {
        self.id
    }
}

// TODO: REMOVE CLONE AND COPY DERIVE
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardID(usize);
impl CardID {
    pub(crate) const fn new(id: usize) -> Self {
        CardID(id)
    }
    pub(crate) fn clone_id(&self) -> Self {
        CardID::new(self.0)
    }
}
impl std::fmt::Display for CardID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct CardBuilder {
    next_id: usize,
}

impl CardBuilder {
    pub(crate) fn new() -> Self {
        CardBuilder { next_id: 0 }
    }
    pub fn build<Kind>(&mut self, kind: Kind) -> Card<Kind> {
        let id = CardID::new(self.next_id);
        self.next_id += 1;
        Card { id, kind }
    }
}
