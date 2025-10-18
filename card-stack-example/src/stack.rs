#[cfg(test)]
use crate::resolvers::NoRequirement;
use crate::{
    actions::{deal_damage::DealDamage, heal::Heal},
    game::{Game, GetStateMut},
    identifications::CharacterID,
    requirements::TargetCharacter,
};
use card_game::stack::{actions::ActionSource, requirements::RequirementAction};
#[cfg(test)]
use card_game::stack::{actions::IncitingActionInfo, priority::PriorityStack};

pub enum Action {
    DealDamage(DealDamage),
    Heal(Heal),
}

impl From<DealDamage> for Action {
    fn from(deal_damage: DealDamage) -> Self {
        Action::DealDamage(deal_damage)
    }
}
impl From<Heal> for Action {
    fn from(heal: Heal) -> Self {
        Action::Heal(heal)
    }
}

pub enum IncitingAction {
    Action(Action),
}

impl From<Action> for IncitingAction {
    fn from(action: Action) -> Self {
        IncitingAction::Action(action)
    }
}

pub enum StackAction {
    Action(Action),
    #[cfg(test)]
    RemoveCharacters(RemoveCharacters),
}
impl From<Action> for StackAction {
    fn from(action: Action) -> Self {
        StackAction::Action(action)
    }
}

#[cfg(test)]
pub struct RemoveCharacters(CharacterID);
impl Default for RemoveCharacters {
    fn default() -> Self {
        RemoveCharacters(CharacterID::new(0))
    }
}
impl ActionSource for RemoveCharacters {
    type Source = CharacterID;
    fn source(&self) -> &Self::Source {
        &self.0
    }
}
#[cfg(test)]
impl<State: GetStateMut<Game>, IncitingAction: IncitingActionInfo<State>>
    card_game::stack::actions::StackAction<State, crate::resolvers::NoInput, IncitingAction>
    for RemoveCharacters
{
    type Requirement = NoRequirement;
    type Resolved = PriorityStack<State, IncitingAction>;
    fn resolve(
        self,
        mut priority: card_game::stack::priority::PriorityMut<
            card_game::stack::priority::PriorityStack<State, IncitingAction>,
        >,
        _: <<Self::Requirement as card_game::stack::requirements::ActionRequirement<
            card_game::stack::priority::PriorityStack<State, IncitingAction>,
            crate::resolvers::NoInput,
        >>::Filter as state_validation::StateFilter<
            card_game::stack::priority::PriorityStack<State, IncitingAction>,
            crate::resolvers::NoInput,
        >>::ValidOutput,
    ) -> Self::Resolved {
        let state = priority.state_mut().state_mut();
        state.characters.remove(&CharacterID::new(0)).unwrap();
        state.characters.remove(&CharacterID::new(1)).unwrap();
        priority.take_priority()
    }
}
