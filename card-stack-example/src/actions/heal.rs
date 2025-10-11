use card_game::stack::{
    actions::ActionSource,
    priority::{Priority, PriorityMut, PriorityStack},
};

use crate::{
    actions::FulfilledStateAction,
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
    type Requirement = TargetCharacter;
    type Stackable = StackAction<State>;
    type Resolved = ResolvedIncitingAction<State, IncitingAction<State>>;
    fn resolve(self, mut priority: PriorityMut<Priority<State>>) -> Self::Resolved {
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
    type Requirement = TargetCharacter;
    fn requirement(&self) -> Self::Requirement {
        TargetCharacter
    }

    type Resolved = ResolvedStackAction<State, IncitingAction>;
    fn resolve(
        self,
        mut priority: PriorityMut<PriorityStack<State, IncitingAction>>,
    ) -> Self::Resolved {
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
