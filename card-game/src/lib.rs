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
    type Game;
    fn generate(
        player_id_builder: PlayerIDBuilder,
        card_builder: CardBuilder,
        generation_data: Self::GenerationData,
    ) -> Self::Game;
    fn new(data: Self::GenerationData) -> Self::Game {
        Self::generate(PlayerIDBuilder::new(), CardBuilder::new(), data)
    }
}
