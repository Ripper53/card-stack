#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
pub use card_game_derive::*;
pub use card_stack as stack;
pub use indexmap;
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

#[macro_export]
macro_rules! card_game_struct {
    (struct $name: ident {
        player: $player: ty,
        event_manager: $event_manager: ty,
        card_description: $card_description: ty,
        zones: $zones: ty,
        $($extra_field_name: ident, $extra_field_ty: ty),* $(,)?
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            player_manager: ::card_game::identifications::PlayerManager<$player>,
            card_manager: ::card_game::cards::CardManager<$event_manager, $card_description>,
            zones: ::card_game::zones::ZoneManager<$zones>,
            $($extra_field_name: $extra_field_ty),*
        }
    };
}

#[macro_export]
macro_rules! define_zone {
    (_zone: $name: ident $({$($field_name: ident: $field_type: ty),* $(,)?})? contains $card_type: ty) => {
        #[derive(Debug, Default, Clone)]
        pub struct $name {
            cards: ::card_game::indexmap::IndexMap<
                ::card_game::cards::CardID,
                ::card_game::cards::Card<$card_type>,
            >,
            $($($field_name: $field_type),*)?
        }
        impl ::card_game::zones::Zone for $name {
            type CardFilter = Self;
            type CardKind = $card_type;
            fn cards(
                &self,
            ) -> impl ::std::iter::Iterator<Item = &::card_game::cards::Card<Self::CardKind>> {
                self.cards.iter().map(|(_, card)| card)
            }
            fn filled_count(&self) -> usize {
                self.cards.len()
            }
            fn get_card(
                &self,
                card_id: ::card_game::cards::CardID,
            ) -> ::std::option::Option<&::card_game::cards::Card<Self::CardKind>> {
                self.cards.get(&card_id)
            }
            fn get_card_mut(
                &mut self,
                card_id: ::card_game::identifications::MutID<::card_game::cards::CardID>,
            ) -> ::std::option::Option<&mut ::card_game::cards::Card<Self::CardKind>> {
                self.cards.get_mut(card_id.id())
            }
            fn get_card_index(
                &self,
                card_id: ::card_game::cards::CardID,
            ) -> ::std::option::Option<usize> {
                self.cards.get_index_of(&card_id)
            }
            fn get_card_from_index(
                &self,
                index: usize,
            ) -> ::std::option::Option<&::card_game::cards::Card<Self::CardKind>> {
                self.cards.get_index(index).map(|(_, card)| card)
            }
        }
        impl ::card_game::zones::ArrayZone for $name {
            fn remove_card(
                &mut self,
                zone_card_id: ::card_game::identifications::ValidCardID<Self::CardFilter>,
            ) -> ::card_game::cards::Card<Self::CardKind> {
                self.cards.shift_remove(&zone_card_id.id()).unwrap()
            }
        }
    };
    (finite $name: ident $({$($field_name: ident: $field_type: ty),* $(,)?})? contains $card_type: ty, max of $max_count: expr $(,)?) => {
        ::card_game::define_zone!(_zone: $name $({$($field_name: $field_type),*})? contains $card_type);
        impl ::card_game::zones::FiniteZone for $name {
            fn max_count(&self) -> usize {
                $max_count
            }
            fn add_card_unchecked(&mut self, card: ::card_game::cards::Card<Self::CardKind>) {
                let _ = self.cards.insert(card.id(), card).unwrap();
            }
        }
    };
    (infinite $name: ident $({$($field_name: ident: $field_type: ty),* $(,)?})? contains $card_type: ty $(,)?) => {
        ::card_game::define_zone!(_zone: $name $({$($field_name: $field_type),*})? contains $card_type);
        impl ::card_game::zones::InfiniteZone for $name {
            fn add_card(&mut self, card: ::card_game::cards::Card<Self::CardKind>) {
                let _ = self.cards.insert(card.id(), card);
            }
        }
    };
    ($($zone_type: ident $name: ident $({$($field_name: ident: $field_type: ty),* $(,)?})? contains $card_type: ty, $(max of $max_count: expr)?),* $(,)?) => {
        $(::card_game::define_zone!($zone_type $name $({$($field_name: $field_type),*})? contains $card_type, $(max of $max_count)?);)*
    };
}

#[macro_export]
macro_rules! define_ability {
    ($ability_data: ty, $description_type: ty, $(events: ty, $description: expr),+ $(,)?) => {
        impl<State: GetState<Game>> ::card_game::events::EventListenerConstructor<State, Play>
            for $ability_data
        where
            PriorityMut<State>: GetStateMut<Game>,
            Play: Event<PriorityMut<State>>,
            StatsChangedEvent: Event<PriorityMut<State>>,
            TriggeredEvent<State, StatsChangedEvent>: NewTriggeredEvent<State, StatsChangedEvent>,
        {
            type Input = StatsBoost;
            fn new_listener(source_card_id: SourceCardID, stats_boost: Self::Input) -> Self {
                BuffPlayed {
                    source_card_id,
                    stats_boost,
                }
            }
        }
    };
}
