use card_game::stack::{
    actions::ActionSource,
    priority::{GetState, Priority, PriorityStack},
    requirements::{ActionRequirement, RequirementAction, TryNewRequirementActionError},
};
use state_validation::StateFilterInput;

use crate::{
    actions::{deal_damage::DealDamage, heal::Heal},
    game::{Game, GetStateMut},
    identifications::CharacterID,
    stack::{Action, StackAction},
};

pub struct Resolver;

pub struct NoInput;
pub struct NoRequirement;
impl<State> ActionRequirement<State, NoInput> for NoRequirement {
    type Filter = ();
    fn collect_inputs(
        _state: &State,
    ) -> state_validation::CollectedInputs<State, impl Iterator<Item = NoInput>> {
        unreachable!();
        state_validation::CollectedInputs::new(Vec::new().into_iter())
    }
}
impl<
    State,
    IncitingAction: card_game::stack::actions::IncitingAction<State, NoInput, Requirement = NoRequirement>,
> card_game::stack::priority::IncitingResolver<State, NoInput, IncitingAction> for Resolver
{
    type Resolved = IncitingAction::Resolved;
    fn resolve_inciting(
        priority: card_game::stack::priority::PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved {
        action.resolve(priority, NoInput)
    }
}
impl<
    State,
    Input: StateFilterInput,
    IncitingAction: card_game::stack::actions::IncitingAction<State, Input>,
> card_game::stack::priority::IncitingResolver<State, Input, IncitingAction> for Resolver
{
    type Resolved = Result<
        RequirementAction<Priority<State>, Input, IncitingAction>,
        TryNewRequirementActionError<Priority<State>, IncitingAction>,
    >;
    fn resolve_inciting(
        priority: card_game::stack::priority::PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved {
        RequirementAction::<Priority<State>, Input, IncitingAction>::try_new(
            priority.take_priority(),
            action,
        )
    }
}
impl<
    State: GetStateMut<Game>,
    IncitingAction: card_game::stack::actions::IncitingActionInfo<State, Stackable = crate::stack::StackAction>,
> card_game::stack::priority::StackResolver<State, IncitingAction> for Resolver
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
                Action::Heal(heal) => {
                    match RequirementAction::<PriorityStack<State, IncitingAction>, _, _>::try_new(
                        priority.take_priority(),
                        heal,
                    ) {
                        Ok(requirement) => card_game::stack::priority::Resolve::Halt(
                            HaltStack::HealRequirement(requirement),
                        ),
                        Err(e) => card_game::stack::priority::Resolve::Continue(e.priority),
                    }
                }
            },
            #[cfg(test)]
            StackAction::RemoveCharacters(remove_characters) => {
                use card_game::stack::actions::StackAction;
                card_game::stack::priority::Resolve::Continue(
                    remove_characters.resolve(priority, NoInput),
                )
            }
        }
    }
}
pub enum HaltStack<Priority> {
    DealDamageRequirement(RequirementAction<Priority, CharacterID, DealDamage>),
    HealRequirement(RequirementAction<Priority, CharacterID, Heal>),
}
