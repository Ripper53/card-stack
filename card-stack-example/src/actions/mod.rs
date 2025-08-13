use deal_damage::DealDamage;
use heal::Heal;

use card_game::stack::{
    actions::{ActionSource, IncitingAction},
    priority::{IncitingPriority, Priority, PriorityStack, StackPriority},
    requirements::FulfilledAction,
};

use crate::{
    game::TurnState,
    identifications::CharacterID,
    requirements::TargetCharacter,
    stack::{Action, StackAction},
};

pub mod deal_damage;
pub mod heal;

pub struct FulfilledStateAction<State, Action: ActionSource, Value> {
    fulfilled_action: FulfilledAction<Action, Value>,
    _state: std::marker::PhantomData<State>,
}
impl<State, Action: ActionSource, Value> FulfilledStateAction<State, Action, Value> {
    pub fn action(&self) -> &FulfilledAction<Action, Value> {
        &self.fulfilled_action
    }
    pub fn take_action(self) -> FulfilledAction<Action, Value> {
        self.fulfilled_action
    }
}
impl<State: Send + Sync, Action: ActionSource, Value: Send + Sync> ActionSource
    for FulfilledStateAction<State, Action, Value>
{
    type Source = Action::Source;
}
impl<State, Action: ActionSource, Value> From<FulfilledAction<Action, Value>>
    for FulfilledStateAction<State, Action, Value>
{
    fn from(fulfilled_action: FulfilledAction<Action, Value>) -> Self {
        FulfilledStateAction {
            fulfilled_action,
            _state: std::marker::PhantomData::default(),
        }
    }
}

pub enum ResolvedIncitingAction<State, IncitingAction: crate::actions::IncitingAction<State>> {
    Complete(Priority<State>),
    Continue(PriorityStack<State, IncitingAction>),
}
pub enum ResolvedStackAction<State, IncitingAction: crate::actions::IncitingAction<State>> {
    Continue(PriorityStack<State, IncitingAction>),
}

pub struct StackDamageAndHeal;
impl ActionSource for StackDamageAndHeal {
    type Source = CharacterID;
}
impl<State: TurnState> IncitingAction<State>
    for FulfilledStateAction<State, StackDamageAndHeal, (CharacterID, CharacterID)>
{
    type EmptyStackRequirement = (TargetCharacter, TargetCharacter);
    type Stackable = StackAction<State>;
    fn requirement(&self) -> Self::EmptyStackRequirement {
        (TargetCharacter, TargetCharacter)
    }

    type ResolvedIncitingAction =
        ResolvedIncitingAction<State, FulfilledStateAction<State, DealDamage, CharacterID>>;
    fn resolve(
        self,
        priority: card_game::stack::priority::PriorityMut<Priority<State>>,
    ) -> Self::ResolvedIncitingAction {
        let (target_0, target_1) = *self.action().value();
        let priority = priority.stack(FulfilledStateAction::from(FulfilledAction::<
            DealDamage,
            CharacterID,
        >::new(
            DealDamage::new(1),
            *self.action().source(),
            target_0,
        )));
        let priority = priority.stack(Action::Heal(FulfilledStateAction::from(FulfilledAction::<
            Heal,
            CharacterID,
        >::new(
            Heal::new(1),
            *self.action().source(),
            target_1,
        ))));
        ResolvedIncitingAction::Continue(priority.take_priority())
    }
}
