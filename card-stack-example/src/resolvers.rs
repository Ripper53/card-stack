use card_game::stack::{
    actions::ActionSource,
    priority::{GetState, Priority, PriorityStack},
    requirements::RequirementAction,
};
use state_validation::StateFilterInput;

use crate::{
    actions::deal_damage::DealDamage,
    game::{Game, GetStateMut},
    identifications::CharacterID,
    stack::{Action, StackAction},
};

pub struct Resolver<T>(std::marker::PhantomData<T>);

impl<State: GetStateMut<Game>, IncitingAction: card_game::stack::actions::IncitingAction<State, ()>>
    card_game::stack::priority::IncitingResolver<State, IncitingAction> for Resolver<()>
{
    type Resolved = IncitingAction::Resolved;
    fn resolve_inciting(
        priority: card_game::stack::priority::PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved {
        action.resolve(priority, ())
    }
}
pub trait IntoHaltStack<Priority> {
    fn into_halt_stack(self, priority: Priority);
}
impl<
    State: GetStateMut<Game>,
    Input: StateFilterInput,
    IncitingAction: IntoHaltStack<Priority<State>>,
> card_game::stack::priority::IncitingResolver<State, IncitingAction> for Resolver<Input>
{
    type Resolved = HaltStack<Priority<State>>;
    fn resolve_inciting(
        priority: card_game::stack::priority::PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved {
        action.into_halt_stack(priority)
    }
}
impl<
    Input: StateFilterInput,
    State: GetStateMut<Game>,
    IncitingAction: card_game::stack::actions::IncitingAction<State, Input, Stackable = crate::stack::StackAction>,
> card_game::stack::priority::StackResolver<State, IncitingAction> for Resolver<Input>
{
    type HaltStack = HaltStack<PriorityStack<State, IncitingAction>>;
    fn resolve_stack(
        priority: card_game::stack::priority::PriorityMut<PriorityStack<State, IncitingAction>>,
        action: <IncitingAction as card_game::stack::actions::IncitingActionInfo<State>>::Stackable,
    ) -> card_game::stack::priority::Resolve<PriorityStack<State, IncitingAction>, Self::HaltStack>
    {
        match action {
            StackAction::Action(action) => match action {
                Action::DealDamage(deal_damage) => {
                    match RequirementAction::<PriorityStack<State, IncitingAction>, _, _>::try_new(
                        priority.take_priority(),
                        deal_damage,
                    ) {
                        Ok(requirement) => card_game::stack::priority::Resolve::Halt(
                            HaltStack::DealDamageRequirement(requirement),
                        ),
                        Err(e) => card_game::stack::priority::Resolve::Continue(e.priority),
                    }
                }
                Action::Heal(heal) => todo!(),
            },
        }
    }
}
pub enum HaltStack<Priority> {
    DealDamageRequirement(RequirementAction<Priority, CharacterID, DealDamage>),
}
