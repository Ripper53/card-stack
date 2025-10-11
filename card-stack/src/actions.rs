use state_validation::{StateFilter, StateFilterInput};

use crate::{
    priority::{Priority, PriorityMut, PriorityStack},
    requirements::ActionRequirement,
};

pub trait ActionSource: Send + Sync + Sized {
    /// Where this action originates from.
    type Source: Send + Sync;
}
/// An action that must be put on an empty stack.
///
/// **NOTE:** if it implements `StackAction` in addition to this trait,
/// it can be put both on an empty stack and stacked stack.
pub trait IncitingAction<State, Input: StateFilterInput>:
    IncitingStackable<State> + ActionSource
{
    /// Requirement must be satisfied before this action can be resolved.
    type Requirement: ActionRequirement<Priority<State>, Input, Self>;

    type Resolved;
    fn resolve(
        self,
        priority: PriorityMut<Priority<State>>,
        input: <<Self::Requirement as ActionRequirement<
            Priority<State>,
            Input,
            Self,
        >>::Filter as StateFilter<Priority<State>, Input>>::ValidOutput,
    ) -> Self::Resolved;
}
pub trait IncitingStackable<State> {
    type Stackable;
}

/// An action that must be put on a stacked stack,
///
/// **NOTE:** if it implements `IncitingAction` in addition to this trait,
/// it can be put both on an empty stack and stacked stack.
pub trait StackAction<
    State,
    Input: StateFilterInput,
    IncitingAction: crate::actions::IncitingStackable<State>,
>: ActionSource
{
    /// Requirement must be satisfied before this action can be resolved.
    type Requirement: ActionRequirement<PriorityStack<State, IncitingAction>, Input, Self>;

    type Resolved;
    fn resolve(
        self,
        priority: PriorityMut<PriorityStack<State, IncitingAction>>,
        input: <<Self::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
            Self,
        >>::Filter as StateFilter<PriorityStack<State, IncitingAction>, Input>>::ValidOutput,
    ) -> Self::Resolved;
}
