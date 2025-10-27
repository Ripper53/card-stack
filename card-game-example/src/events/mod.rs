use card_game::events::{GetEventManager, GetEventManagerMut};

use crate::{
    Game,
    events::summon::{NormalSummoned, SpecialSummoned, Summoned},
    steps::GetStateMut,
    valid_actions::PassiveGiveAttack,
};

pub mod summon;

pub struct EventManager {
    summoned: card_game::events::EventManager<Game, Summoned, Game>,
    normal_summoned: card_game::events::EventManager<Game, NormalSummoned, Game>,
    special_summoned: card_game::events::EventManager<Game, SpecialSummoned, Game>,
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager {
            summoned: card_game::events::EventManager::empty(),
            normal_summoned: card_game::events::EventManager::empty(),
            special_summoned: card_game::events::EventManager::empty(),
        }
    }
}

impl GetEventManager<Game, Summoned> for EventManager {
    type Output = Game;
    fn event_manager(&self) -> card_game::events::EventManager<Game, Summoned, Self::Output> {
        card_game::events::EventManager::<Game, Summoned, Self::Output>::new_combined(
            &self.summoned,
            &self.normal_summoned,
        )
        .combine(&self.special_summoned)
    }
}
impl GetEventManager<Game, NormalSummoned> for EventManager {
    type Output = Game;
    fn event_manager(&self) -> card_game::events::EventManager<Game, NormalSummoned, Self::Output> {
        self.normal_summoned.clone()
    }
}
impl GetEventManagerMut<Game, NormalSummoned> for EventManager {
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, NormalSummoned, Self::Output> {
        &mut self.normal_summoned
    }
}
impl GetEventManager<Game, SpecialSummoned> for EventManager {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<Game, SpecialSummoned, Self::Output> {
        self.special_summoned.clone()
    }
}
impl GetEventManagerMut<Game, SpecialSummoned> for EventManager {
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, SpecialSummoned, Self::Output> {
        &mut self.special_summoned
    }
}
