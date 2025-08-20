#[cfg(feature = "derive")]
pub use card_game_derive::*;
pub use card_stack as stack;

use crate::{cards::CardBuilder, identifications::PlayerIDBuilder};

pub mod abilities;
pub mod cards;
pub mod commands;
pub mod identifications;
pub mod steps;
pub mod zones;

pub trait CardGameBuilder: Sized {
    type GenerationData;
    fn generate(
        player_id_builder: PlayerIDBuilder,
        card_builder: CardBuilder,
        generation_data: Self::GenerationData,
    ) -> Self;
    fn new(data: Self::GenerationData) -> Self {
        Self::generate(PlayerIDBuilder::new(), CardBuilder::new(), data)
    }
}
pub struct CardGame<T>(T);
impl<T> CardGame<T> {
    pub fn context<R>(self, f: impl FnOnce(T) -> R) -> CardGame<R> {
        CardGame(f(self.0))
    }
}

pub struct ContextID<ID>(ID);

impl<ID> ContextID<ID> {
    pub fn take_id(self) -> ID {
        self.0
    }
}
