use card_game::events::{GetEventManager, GetEventManagerMut};

use crate::{
    Game, events::summon::SpecialSummoned, steps::GetStateMut, valid_actions::PassiveGiveAttack,
};

pub mod summon;

pub struct EventManager {
    pub(crate) passive_give_attack:
        card_game::events::EventManager<Game, SpecialSummoned, PassiveGiveAttack>,
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager {
            passive_give_attack: card_game::events::EventManager::new(),
        }
    }
}

impl GetEventManager<Game, SpecialSummoned, PassiveGiveAttack> for EventManager {
    fn event_manager(
        &self,
    ) -> &card_game::events::EventManager<Game, SpecialSummoned, PassiveGiveAttack> {
        &self.passive_give_attack
    }
}
impl GetEventManagerMut<Game, SpecialSummoned, PassiveGiveAttack> for EventManager {
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<Game, SpecialSummoned, PassiveGiveAttack> {
        &mut self.passive_give_attack
    }
}
