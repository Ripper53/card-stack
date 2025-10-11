use crate::{
    actions::{
        FulfilledStateAction, ResolvedIncitingAction, ResolvedStackAction, deal_damage::DealDamage,
        heal::Heal,
    },
    identifications::CharacterID,
    requirements::TargetCharacter,
};
use card_game::stack::{actions::ActionSource, requirements::RequirementAction};

pub enum Action<State> {
    DealDamage(RequirementAction<State, CharacterID, DealDamage>),
    Heal(RequirementAction<State, CharacterID, Heal>),
}

impl<State> From<FulfilledStateAction<State, CharacterID, DealDamage>> for Action<State> {
    fn from(deal_damage: RequirementAction<State, CharacterID, DealDamage>) -> Self {
        Action::DealDamage(deal_damage)
    }
}
impl<State> From<RequirementAction<State, CharacterID, Heal>> for Action<State> {
    fn from(heal: FulfilledStateAction<State, CharacterID, Heal>) -> Self {
        Action::Heal(heal)
    }
}

pub enum IncitingAction<State> {
    Action(Action<State>),
}

impl<State> From<Action<State>> for IncitingAction<State> {
    fn from(action: Action<State>) -> Self {
        IncitingAction::Action(action)
    }
}

impl<State: Send + Sync> ActionSource for IncitingAction<State> {
    type Source = CharacterID;
}
impl<State: TurnState> card_game::stack::actions::IncitingAction<State> for IncitingAction<State> {
    type Requirement = TargetCharacter;
    fn requirement(&self) -> Self::Requirement {
        TargetCharacter
    }

    type Stackable = StackAction<State>;
    type Resolved = ResolvedIncitingAction<State, Self>;
    fn resolve(
        self,
        priority: card_game::stack::priority::PriorityMut<
            card_game::stack::priority::Priority<State>,
        >,
    ) -> Self::Resolved {
        match self {
            IncitingAction::Action(action) => match action {
                Action::DealDamage(deal_damage) => {
                    card_game::stack::actions::IncitingAction::resolve(deal_damage, priority)
                }
                Action::Heal(heal) => {
                    card_game::stack::actions::IncitingAction::resolve(heal, priority)
                }
            },
        }
    }
}

pub enum StackAction<State> {
    Action(Action<State>),
}
impl<State> From<Action<State>> for StackAction<State> {
    fn from(action: Action<State>) -> Self {
        StackAction::Action(action)
    }
}

impl<State: Send + Sync> ActionSource for StackAction<State> {
    type Source = CharacterID;
}
impl<State: TurnState, IncitingAction: card_game::stack::actions::IncitingAction<State>>
    card_game::stack::actions::StackAction<State, IncitingAction> for StackAction<State>
{
    type Requirement = TargetCharacter;
    fn requirement(&self) -> Self::Requirement {
        TargetCharacter
    }

    type Resolved = ResolvedStackAction<State, IncitingAction>;
    fn resolve(
        self,
        priority: card_game::stack::priority::PriorityMut<
            card_game::stack::priority::PriorityStack<State, IncitingAction>,
        >,
    ) -> Self::Resolved {
        match self {
            StackAction::Action(action) => match action {
                Action::DealDamage(deal_damage) => {
                    card_game::stack::actions::StackAction::resolve(deal_damage, priority)
                }
                Action::Heal(heal) => {
                    card_game::stack::actions::StackAction::resolve(heal, priority)
                }
            },
        }
    }
}
