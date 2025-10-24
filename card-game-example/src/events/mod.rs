use card_game::events::{GetEventManager, GetEventManagerMut};

use crate::{
    Game, events::summon::SpecialSummoned, steps::GetStateMut, valid_actions::PassiveGiveAttack,
};

pub mod summon;

pub struct EventManager {
    special_summoned: card_game::events::EventManager<Game, SpecialSummoned, Game>,
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager {
            special_summoned: card_game::events::EventManager::empty(),
        }
    }
}

impl GetEventManager<Game, SpecialSummoned> for EventManager {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> &card_game::events::EventManager<Game, SpecialSummoned, Self::Output> {
        &self.special_summoned
    }
}
impl GetEventManagerMut<Game, SpecialSummoned> for EventManager {
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, SpecialSummoned, Self::Output> {
        &mut self.special_summoned
    }
}
