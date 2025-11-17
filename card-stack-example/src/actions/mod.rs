use deal_damage::DealDamage;
use heal::Heal;

use card_game::stack::{
    actions::{ActionSource, IncitingAction, IncitingActionInfo},
    priority::{GetState, Priority, PriorityStack},
    requirements::{FulfilledAction, RequirementAction},
};

use crate::{
    game::{Game, GetStateMut},
    identifications::CharacterID,
    requirements::TargetCharacter,
    stack::{Action, StackAction},
};

pub mod deal_damage;
pub mod heal;

#[derive(Debug)]
pub struct StackDamageAndHeal {
    source: CharacterID,
}
impl StackDamageAndHeal {
    pub fn new(source: CharacterID) -> Self {
        StackDamageAndHeal { source }
    }
}
impl ActionSource for StackDamageAndHeal {
    type Source = CharacterID;
    fn source(&self) -> &Self::Source {
        &self.source
    }
}
impl<State: GetStateMut<Game>> IncitingAction<State, ()> for StackDamageAndHeal {
    type Requirement = ();
    fn resolve(
        self,
        mut priority: card_game::stack::priority::PriorityMut<Priority<State>>,
        _: <<Self::Requirement as card_game::stack::requirements::ActionRequirement<
            Priority<State>,
            (),
        >>::Filter as state_validation::StateFilter<Priority<State>, ()>>::ValidOutput,
    ) -> Self::Resolved {
        let priority = priority.stack(Heal::new(self.source, 1));
        let priority = priority.stack(StackAction::Action(Action::DealDamage(DealDamage::new(
            self.source,
            1,
        ))));
        priority.take_priority()
    }
}
impl<State: GetStateMut<Game>> IncitingActionInfo<State> for StackDamageAndHeal {
    type Resolved = PriorityStack<State, Heal>;
    type Stackable = StackAction;
}
