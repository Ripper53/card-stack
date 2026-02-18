#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
pub use card_game_derive::*;
pub use card_stack as stack;
//pub use state_validation as validation;
pub use variadics_please;

use crate::{cards::CardManager, identifications::PlayerIDBuilder};

pub mod abilities;
pub mod cards;
pub mod commands;
mod context;
pub mod events;
pub mod identifications;
pub mod zones;
pub use context::*;

pub trait CardGameBuilder<EventManager: Default, Description>: Sized {
    type GenerationData;
    type Game;
    fn generate(
        player_id_builder: PlayerIDBuilder,
        card_manager: CardManager<EventManager, Description>,
        generation_data: Self::GenerationData,
    ) -> Self::Game;
    fn new(data: Self::GenerationData) -> Self::Game {
        Self::generate(
            PlayerIDBuilder::new(),
            CardManager::new(EventManager::default()),
            data,
        )
    }
}
