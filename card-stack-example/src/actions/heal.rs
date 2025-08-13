use card_game::stack::{
    actions::ActionSource,
    priority::{Priority, PriorityMut, PriorityStack},
};

use crate::{
    actions::FulfilledStateAction,
    game::TurnState,
    identifications::CharacterID,
    requirements::TargetCharacter,
    stack::{IncitingAction, StackAction},
};

use super::{ResolvedIncitingAction, ResolvedStackAction};

pub struct Heal {
    amount: usize,
}
impl Heal {
    pub fn new(amount: usize) -> Self {
        Heal { amount }
    }
}
impl ActionSource for Heal {
    type Source = CharacterID;
}

impl<State: TurnState> card_game::stack::actions::IncitingAction<State>
    for FulfilledStateAction<State, Heal, CharacterID>
{
    type EmptyStackRequirement = TargetCharacter;
    fn requirement(&self) -> Self::EmptyStackRequirement {
        TargetCharacter
    }
    type Stackable = StackAction<State>;
    type ResolvedIncitingAction = ResolvedIncitingAction<State, IncitingAction<State>>;
    fn resolve(self, mut priority: PriorityMut<Priority<State>>) -> Self::ResolvedIncitingAction {
        let character = priority
            .state_mut()
            .game_mut()
            .characters
            .get_mut(self.action().value())
            .unwrap();
        character.health += self.take_action().take_action().amount;
        ResolvedIncitingAction::Complete(priority.take_priority())
    }
}
impl<State: TurnState, IncitingAction: card_game::stack::actions::IncitingAction<State>>
    card_game::stack::actions::StackAction<State, IncitingAction>
    for FulfilledStateAction<State, Heal, CharacterID>
{
    type StackedRequirement = TargetCharacter;
    fn requirement(&self) -> Self::StackedRequirement {
        TargetCharacter
    }

    type ResolvedStackAction = ResolvedStackAction<State, IncitingAction>;
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
        character.health += self.take_action().take_action().amount;
        ResolvedStackAction::Continue(priority.take_priority())
    }
}
