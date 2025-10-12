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

pub struct Heal {
    source: CharacterID,
    amount: usize,
}
impl Heal {
    pub fn new(source: CharacterID, amount: usize) -> Self {
        Heal { source, amount }
    }
}
impl ActionSource for Heal {
    type Source = CharacterID;
    fn source(&self) -> &Self::Source {
        &self.source
    }
}

impl<State: GetStateMut<Game>> card_game::stack::actions::IncitingAction<State, CharacterID>
    for Heal
{
    type Requirement = TargetCharacter;
    fn resolve(
        self,
        mut priority: PriorityMut<Priority<State>>,
        input: <<Self::Requirement as card_game::stack::requirements::ActionRequirement<
                Priority<State>,
                CharacterID,
            >>::Filter as card_game::validation::StateFilter<Priority<State>, CharacterID>>::ValidOutput,
    ) -> Self::Resolved {
        let character = priority
            .state_mut()
            .state_mut()
            .characters
            .get_mut(&input)
            .unwrap();
        character.health += self.amount;
        priority.take_priority()
    }
}
impl<State: GetState<Game>> card_game::stack::actions::IncitingActionInfo<State> for Heal {
    type Resolved = Priority<State>;
    type Stackable = StackAction;
}
impl<State: GetStateMut<Game>, IncitingAction: card_game::stack::actions::IncitingActionInfo<State>>
    card_game::stack::actions::StackAction<State, CharacterID, IncitingAction> for Heal
{
    type Requirement = TargetCharacter;
    type Resolved = PriorityStack<State, IncitingAction>;
    fn resolve(
        self,
        mut priority: PriorityMut<PriorityStack<State, IncitingAction>>,
        input: <<Self::Requirement as card_game::stack::requirements::ActionRequirement<
            PriorityStack<State, IncitingAction>,
            CharacterID,
        >>::Filter as card_game::validation::StateFilter<
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
        character.health += self.amount;
        priority.take_priority()
    }
}
