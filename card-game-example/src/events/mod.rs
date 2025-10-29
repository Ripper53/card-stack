use card_game::events::{GetEventManager, GetEventManagerMut};

use crate::{
    Game,
    events::summon::{NormalSummoned, SpecialSummoned, Summoned},
    steps::{GetStateMut, MainStep},
    valid_actions::PassiveGiveAttack,
};

pub mod summon;

pub struct EventManager {
    summoned: card_game::events::EventManager<Game, Summoned, Game>,
    summoned_during_mainstep: card_game::events::EventManager<MainStep, Summoned, Game>,
    normal_summoned: card_game::events::EventManager<Game, NormalSummoned, Game>,
    special_summoned: card_game::events::EventManager<Game, SpecialSummoned, Game>,
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager {
            summoned: card_game::events::EventManager::empty(),
            summoned_during_mainstep: card_game::events::EventManager::empty(),
            normal_summoned: card_game::events::EventManager::empty(),
            special_summoned: card_game::events::EventManager::empty(),
        }
    }
}

impl GetEventManagerMut<Game, Summoned> for EventManager {
    type Output = Game;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, Summoned, Self::Output> {
        &mut self.summoned
    }
}
impl GetEventManagerMut<MainStep, Summoned> for EventManager {
    type Output = Game;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<MainStep, Summoned, Self::Output> {
        &mut self.summoned_during_mainstep
    }
}
impl GetEventManager<MainStep, NormalSummoned> for EventManager {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, NormalSummoned, Self::Output> {
        card_game::events::EventManager::<MainStep, NormalSummoned, Self::Output>::new_combined(
            &self.summoned_during_mainstep,
            &self.summoned,
        )
        .combine(&self.normal_summoned)
    }
}
impl GetEventManager<Game, NormalSummoned> for EventManager {
    type Output = Game;
    fn event_manager(&self) -> card_game::events::EventManager<Game, NormalSummoned, Self::Output> {
        card_game::events::EventManager::<Game, NormalSummoned, Self::Output>::new_combined(
            &self.normal_summoned,
            &self.summoned,
        )
    }
}
impl GetEventManagerMut<Game, NormalSummoned> for EventManager {
    type Output = Game;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, NormalSummoned, Self::Output> {
        &mut self.normal_summoned
    }
}

impl GetEventManager<MainStep, SpecialSummoned> for EventManager {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, SpecialSummoned, Self::Output> {
        card_game::events::EventManager::<MainStep, SpecialSummoned, Self::Output>::new_combined(
            &self.special_summoned,
            &self.summoned,
        )
        .combine(&self.summoned_during_mainstep)
    }
}
impl GetEventManager<Game, SpecialSummoned> for EventManager {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<Game, SpecialSummoned, Self::Output> {
        card_game::events::EventManager::<Game, SpecialSummoned, Self::Output>::new_combined(
            &self.special_summoned,
            &self.summoned,
        )
    }
}
impl GetEventManagerMut<Game, SpecialSummoned> for EventManager {
    type Output = Game;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, SpecialSummoned, Self::Output> {
        &mut self.special_summoned
    }
}
