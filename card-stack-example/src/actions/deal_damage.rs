use card_game::stack::{
    actions::ActionSource,
    priority::{GetState, Priority, PriorityMut, PriorityStack},
    requirements::RequirementAction,
};

use crate::{
    actions::{FulfilledStateAction, ResolvedIncitingAction, ResolvedStackAction},
    game::Game,
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
    type Requirement = TargetCharacter;
    type Resolved = ResolvedIncitingAction<State, IncitingAction<State>>;
    fn resolve(self, mut priority: PriorityMut<Priority<State>>) -> Self::Resolved {
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
impl<State: GetState<Game>> card_game::stack::actions::IncitingStackable<State>
    for RequirementAction<State, CharacterID, DealDamage>
{
    type Stackable = ();
}
impl<State: TurnState, IncitingAction: crate::actions::IncitingAction<State>>
    card_game::stack::actions::StackAction<State, IncitingAction>
    for FulfilledStateAction<State, DealDamage, CharacterID>
{
    type Requirement = TargetCharacter;
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
        character.health -= character
            .health
            .min(self.take_action().take_action().damage);
        ResolvedStackAction::Continue(priority.take_priority())
    }
}
