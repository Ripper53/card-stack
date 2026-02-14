use card_game::stack::{
    actions::ActionSource,
    priority::{GetState, Priority, PriorityMut, PriorityStack},
    requirements::RequirementAction,
};

use crate::{
    game::{Game, GetStateMut},
    identifications::CharacterID,
    requirements::TargetCharacter,
    stack::{IncitingAction, StackAction},
};

pub struct DealDamage {
    source: CharacterID,
    damage: usize,
}
impl DealDamage {
    pub fn new(source: CharacterID, damage: usize) -> Self {
        DealDamage { source, damage }
    }
}
impl ActionSource for DealDamage {
    type Source = CharacterID;
    fn source(&self) -> &Self::Source {
        &self.source
    }
}

impl<State: GetStateMut<Game>> card_game::stack::actions::IncitingAction<State, CharacterID>
    for DealDamage
{
    type Requirement = TargetCharacter;
    fn resolve(
        self,
        mut priority: PriorityMut<Priority<State>>,
        input: <<Self::Requirement as card_game::stack::requirements::ActionRequirement<
                Priority<State>,
                CharacterID,
            >>::Filter as state_validation::StateFilter<Priority<State>, CharacterID>>::ValidOutput,
    ) -> Self::Resolved {
        let character = priority
            .state_mut()
            .state_mut()
            .characters
            .get_mut(&input)
            .unwrap();
        character.health -= character.health.min(self.damage);
        priority.take_priority()
    }
}
impl<State: GetState<Game>> card_game::stack::actions::IncitingActionInfo<State> for DealDamage {
    type Resolved = Priority<State>;
    type Stackable = StackAction;
}
impl<State: GetStateMut<Game>, IncitingAction: crate::actions::IncitingActionInfo<State>>
    card_game::stack::actions::StackAction<State, CharacterID, IncitingAction> for DealDamage
{
    type Requirement = TargetCharacter;
    type Resolved = PriorityStack<State, IncitingAction>;
    fn resolve(
        self,
        mut priority: PriorityMut<PriorityStack<State, IncitingAction>>,
        input: <<Self::Requirement as card_game::stack::requirements::ActionRequirement<
            PriorityStack<State, IncitingAction>,
            CharacterID,
        >>::Filter as state_validation::StateFilter<
            PriorityStack<State, IncitingAction>,
            CharacterID,
        >>::ValidOutput,
    ) -> Self::Resolved {
        let character = priority
            .state_mut()
            .state_mut()
            .characters
            .get_mut(&input)
            .unwrap();
        character.health -= character.health.min(self.damage);
        priority.take_priority()
    }
}
