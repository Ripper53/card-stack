use card_game::events::{AddEventListener, GetEventManager};

use crate::{
    Game,
    events::summon::{NormalSummoned, SpecialSummoned, Summoned},
    steps::{GetStateMut, MainStep},
    valid_actions::PassiveGiveAttack,
};

pub mod summon;

pub struct EventManager {
    normal_summoned_during_main_step:
        card_game::events::EventManager<MainStep, NormalSummoned, MainStep>,
    special_summoned_during_main_step:
        card_game::events::EventManager<MainStep, SpecialSummoned, MainStep>,
}

impl Default for EventManager {
    fn default() -> Self {
        EventManager {
            normal_summoned_during_main_step: card_game::events::EventManager::empty(),
            special_summoned_during_main_step: card_game::events::EventManager::empty(),
        }
    }
}

impl AddEventListener<MainStep, Summoned> for EventManager {
    type Output = MainStep;
    fn add_listener<Listener: card_game::events::EventListener<MainStep, Summoned>>(
        &mut self,
        listener: Listener,
    ) where
        <Listener::Action as state_validation::ValidAction<
            MainStep,
            <Listener::Filter as state_validation::StateFilter<
                MainStep,
                <Summoned as card_game::events::Event<MainStep>>::Input,
            >>::ValidOutput,
        >>::Output: Into<Self::Output>,
    {
        self.normal_summoned_during_main_step
            .add_listener(listener.clone());
        self.special_summoned_during_main_step
            .add_listener(listener);
    }
}
impl GetEventManager<MainStep, NormalSummoned> for EventManager {
    type Output = MainStep;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, NormalSummoned, Self::Output> {
        self.normal_summoned_during_main_step.clone()
    }
}
impl AddEventListener<MainStep, NormalSummoned> for EventManager {
    type Output = MainStep;
    fn add_listener<Listener: card_game::events::EventListener<MainStep, NormalSummoned>>(
        &mut self,
        listener: Listener,
    ) where
        <Listener::Action as state_validation::ValidAction<
            MainStep,
            <Listener::Filter as state_validation::StateFilter<
                MainStep,
                <NormalSummoned as card_game::events::Event<MainStep>>::Input,
            >>::ValidOutput,
        >>::Output: Into<Self::Output>,
    {
        self.normal_summoned_during_main_step.add_listener(listener);
    }
}

impl GetEventManager<MainStep, SpecialSummoned> for EventManager {
    type Output = MainStep;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<MainStep, SpecialSummoned, Self::Output> {
        self.special_summoned_during_main_step.clone()
    }
}
impl AddEventListener<MainStep, SpecialSummoned> for EventManager {
    type Output = MainStep;
    fn add_listener<Listener: card_game::events::EventListener<MainStep, SpecialSummoned>>(
        &mut self,
        listener: Listener,
    ) where
        <Listener::Action as state_validation::ValidAction<
            MainStep,
            <Listener::Filter as state_validation::StateFilter<
                MainStep,
                <SpecialSummoned as card_game::events::Event<MainStep>>::Input,
            >>::ValidOutput,
        >>::Output: Into<Self::Output>,
    {
        self.special_summoned_during_main_step
            .add_listener(listener);
    }
}
