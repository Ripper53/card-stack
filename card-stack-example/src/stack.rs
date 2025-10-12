use crate::{
    actions::{deal_damage::DealDamage, heal::Heal},
    game::{Game, GetStateMut},
    identifications::CharacterID,
    requirements::TargetCharacter,
};
use card_game::stack::{actions::ActionSource, requirements::RequirementAction};

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
}
impl From<Action> for StackAction {
    fn from(action: Action) -> Self {
        StackAction::Action(action)
    }
}
