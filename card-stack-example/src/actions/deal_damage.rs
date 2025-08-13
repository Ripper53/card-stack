use card_game::stack::{
    actions::ActionSource,
    priority::{Priority, PriorityMut, PriorityStack},
};

use crate::{
    actions::{FulfilledStateAction, ResolvedIncitingAction, ResolvedStackAction},
    game::TurnState,
    identifications::CharacterID,
    requirements::TargetCharacter,
    stack::{IncitingAction, StackAction},
};

pub struct DealDamage {
    damage: usize,
}
impl DealDamage {
    pub fn new(damage: usize) -> Self {
        DealDamage { damage }
    }
}
impl ActionSource for DealDamage {
    type Source = CharacterID;
}

impl<State: TurnState> card_game::stack::actions::IncitingAction<State>
    for FulfilledStateAction<State, DealDamage, CharacterID>
{
    type Stackable = StackAction<State>;
    type EmptyStackRequirement = TargetCharacter;
    fn requirement(&self) -> Self::EmptyStackRequirement {
        TargetCharacter
    }
    type ResolvedIncitingAction = ResolvedIncitingAction<State, IncitingAction<State>>;
    fn resolve(self, mut priority: PriorityMut<Priority<State>>) -> Self::ResolvedIncitingAction {
        let character = priority
            .state_mut()
            .game_mut()
            .characters
            .get_mut(self.action().value())
            .unwrap();
        character.health -= character
            .health
            .min(self.take_action().take_action().damage);
        ResolvedIncitingAction::Complete(priority.take_priority())
    }
}
impl<State: TurnState, IncitingAction: crate::actions::IncitingAction<State>>
    card_game::stack::actions::StackAction<State, IncitingAction>
    for FulfilledStateAction<State, DealDamage, CharacterID>
{
    type StackedRequirement = TargetCharacter;
    type ResolvedStackAction = ResolvedStackAction<State, IncitingAction>;
    fn requirement(&self) -> Self::StackedRequirement {
        TargetCharacter
    }
    fn resolve(
        self,
        mut priority: PriorityMut<PriorityStack<State, IncitingAction>>,
    ) -> Self::ResolvedStackAction {
        let character = priority
            .state_mut()
            .game_mut()
            .characters
            .get_mut(self.action().value())
            .unwrap();
        character.health -= character
            .health
            .min(self.take_action().take_action().damage);
        ResolvedStackAction::Continue(priority.take_priority())
    }
}
