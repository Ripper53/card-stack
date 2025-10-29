use card_game::events::{GetEventManager, GetEventManagerMut};

use crate::{
    Game,
    events::summon::{NormalSummoned, SpecialSummoned, Summoned},
    steps::{GetStateMut, MainStep},
    valid_actions::PassiveGiveAttack,
};

pub mod summon;

pub struct EventManager {
    summoned_during_mainstep: card_game::events::EventManager<MainStep, Summoned, MainStep>,
    normal_summoned_during_main_step:
        card_game::events::EventManager<MainStep, NormalSummoned, MainStep>,
    special_summoned_during_main_step:
        card_game::events::EventManager<MainStep, SpecialSummoned, MainStep>,
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager {
            summoned_during_mainstep: card_game::events::EventManager::empty(),
            normal_summoned_during_main_step: card_game::events::EventManager::empty(),
            special_summoned_during_main_step: card_game::events::EventManager::empty(),
        }
    }
}

impl GetEventManagerMut<MainStep, Summoned> for EventManager {
    type Output = MainStep;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<MainStep, Summoned, Self::Output> {
        &mut self.summoned_during_mainstep
    }
}
impl GetEventManager<MainStep, NormalSummoned> for EventManager {
    type Output = MainStep;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, NormalSummoned, Self::Output> {
        card_game::events::EventManager::<MainStep, NormalSummoned, Self::Output>::new_combined(
            &self.summoned_during_mainstep,
            &self.normal_summoned_during_main_step,
        )
    }
}
impl GetEventManagerMut<MainStep, NormalSummoned> for EventManager {
    type Output = MainStep;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<MainStep, NormalSummoned, Self::Output> {
        &mut self.normal_summoned_during_main_step
    }
}

impl GetEventManager<MainStep, SpecialSummoned> for EventManager {
    type Output = MainStep;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, SpecialSummoned, Self::Output> {
        card_game::events::EventManager::<MainStep, SpecialSummoned, Self::Output>::new_combined(
            &self.special_summoned_during_main_step,
            &self.summoned_during_mainstep,
        )
    }
}
impl GetEventManagerMut<MainStep, SpecialSummoned> for EventManager {
    type Output = MainStep;
    fn event_manager_mut(
        &mut self,
    ) -> &mut card_game::events::EventManager<MainStep, SpecialSummoned, Self::Output> {
        &mut self.special_summoned_during_main_step
    }
}
