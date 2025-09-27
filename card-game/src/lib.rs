//! # Card Game
//! ## State Filter
//! What is a `StateFilter`?
//! It takes two generic parameters:
//! 1. the state of the game
//! 2. the input for the filter
//!
//! `StateFilter` returns an `Option<Self::ValidOutput>` in its only function: `filter`.
//!
//! A `StateFilter` is ***validation*** for your game state. For example, you might have a `StateFilter` that is implemented for a `struct` named `CardIn`:
//! ```
//! # use std::collections::HashMap;
//! # use card_game::{
//! #     cards::{Card, CardID},
//! #     identifications::{ActivePlayer, GetValidCardIDFromZone, PlayerID, ValidCardID, ValidPlayerID, PlayerManager},
//! #     stack::priority::GetState,
//! #     steps::Step,
//! #     validation::StateFilter,
//! #     zones::{ArrayZone, FiniteZone, InfiniteZone, Zone},
//! # };
//! #
//! # struct Game {
//! #     player_manager: PlayerManager<()>,
//! #     hand_zone: HandZone,
//! # }
//! # impl Default for Game {
//! #     fn default() -> Self {
//! #         Game {
//! #             player_manager: PlayerManager::new(HashMap::new()),
//! #             hand_zone: HandZone::new(PlayerID::new(0)),
//! #         }
//! #     }
//! # }
//! #
//! # struct HandZone {
//! #     player_id: PlayerID,
//! #     cards: HashMap<CardID, Card<()>>,
//! # }
//! #
//! # impl HandZone {
//! #     pub fn new(player_id: PlayerID) -> Self {
//! #         HandZone {
//! #             player_id,
//! #             cards: HashMap::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl FiniteZone for HandZone {
//! #     fn max_count(&self) -> usize {
//! #         10
//! #     }
//! #     fn add_card_unchecked(&mut self, card: Card<Self::CardKind>) {
//! #         self.cards.insert(card.id(), card).unwrap();
//! #     }
//! # }
//! # impl ArrayZone for HandZone {
//! #     fn remove_card(&mut self, zone_card_id: ValidCardID<CardIn<Self>>) -> Card<Self::CardKind> {
//! #         zone_card_id.remove(|id| self.cards.remove(&id.id()))
//! #     }
//! # }
//! # impl Zone for HandZone {
//! #     type CardKind = ();
//! #     type CardFilter = CardIn<Self>;
//! #     fn player_id(&self) -> PlayerID {
//! #         self.player_id
//! #     }
//! #     fn filled_count(&self) -> usize {
//! #         self.cards.len()
//! #     }
//! #     fn get_card(&self, card_id: CardID) -> Option<&Card<Self::CardKind>> {
//! #         self.cards.get(&card_id)
//! #     }
//! #     fn get_card_from_index(&self, _index: usize) -> Option<&Card<Self::CardKind>> {
//! #         unimplemented!()
//! #     }
//! #     fn cards(&self) -> impl Iterator<Item = &Card<Self::CardKind>> {
//! #         self.cards.iter().map(|(card_id, card)| card)
//! #     }
//! # }
//! #
//! struct CardIn<T>(std::marker::PhantomData<T>);
//!
//! /// Validates card is within the hand zone.
//! //      (State of Game)                        (Input for Filter)
//! impl<State: GetState<Game>> StateFilter<State, (PlayerID, CardID)> for CardIn<HandZone> {
//!     type ValidOutput = (ValidPlayerID<()>, ValidCardID<Self>);
//!     fn filter(
//!         state: &State,
//!         (player_id, card_id): (PlayerID, CardID),
//!     ) -> Option<Self::ValidOutput> {
//!         let state = state.state();
//!         let valid_player_id = ValidPlayerID::try_new(&state.player_manager, player_id)?;
//!         let valid_card_id = ValidCardID::try_new(card_id, &state.hand_zone)?;
//!         Some((valid_player_id, valid_card_id))
//!     }
//! }
//! ```
//!
//! Now, we can use `Validator` to run the filter on our game state.
//! ```
//! # use card_game::validation::Validator;
//! # struct MainState { game: Game }
//! let main_step = MainStep {
//!     game: Game::default(),
//! };
//! let validator = Validator::<_, _, CardIn<HandZone>>::try_new(main, (player_id, card_id)).expect("failed validation");
//! ```

#[cfg(feature = "derive")]
pub use card_game_derive::*;
pub use card_stack as stack;
pub use variadics_please;

use crate::{
    cards::{CardBuilder, CardManager},
    identifications::PlayerIDBuilder,
};

pub mod abilities;
pub mod cards;
pub mod commands;
pub mod identifications;
pub mod steps;
pub mod validation;
pub mod zones;

pub trait CardGameBuilder: Sized {
    type GenerationData;
    type Game;
    fn generate(
        player_id_builder: PlayerIDBuilder,
        card_manager: CardManager,
        generation_data: Self::GenerationData,
    ) -> Self::Game;
    fn new(data: Self::GenerationData) -> Self::Game {
        Self::generate(PlayerIDBuilder::new(), CardManager::new(), data)
    }
}
