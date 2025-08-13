use crate::{
    actions::{
        FulfilledStateAction, ResolvedIncitingAction, ResolvedStackAction, deal_damage::DealDamage,
        heal::Heal,
    },
    game::TurnState,
    identifications::CharacterID,
    requirements::TargetCharacter,
};
use card_game::stack::actions::ActionSource;

pub enum Action<State> {
    DealDamage(FulfilledStateAction<State, DealDamage, CharacterID>),
    Heal(FulfilledStateAction<State, Heal, CharacterID>),
}

impl<State> From<FulfilledStateAction<State, DealDamage, CharacterID>> for Action<State> {
    fn from(deal_damage: FulfilledStateAction<State, DealDamage, CharacterID>) -> Self {
        Action::DealDamage(deal_damage)
    }
}
impl<State> From<FulfilledStateAction<State, Heal, CharacterID>> for Action<State> {
    fn from(heal: FulfilledStateAction<State, Heal, CharacterID>) -> Self {
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
    type EmptyStackRequirement = TargetCharacter;
    fn requirement(&self) -> Self::EmptyStackRequirement {
        TargetCharacter
    }

    type Stackable = StackAction<State>;
    type ResolvedIncitingAction = ResolvedIncitingAction<State, Self>;
    fn resolve(
        self,
        priority: card_game::stack::priority::PriorityMut<
            card_game::stack::priority::Priority<State>,
        >,
    ) -> Self::ResolvedIncitingAction {
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
    type StackedRequirement = TargetCharacter;
    fn requirement(&self) -> Self::StackedRequirement {
        TargetCharacter
    }

    type ResolvedStackAction = ResolvedStackAction<State, IncitingAction>;
    fn resolve(
        self,
        priority: card_game::stack::priority::PriorityMut<
            card_game::stack::priority::PriorityStack<State, IncitingAction>,
        >,
    ) -> Self::ResolvedStackAction {
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
