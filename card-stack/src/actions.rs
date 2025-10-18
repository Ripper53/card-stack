use state_validation::{StateFilter, StateFilterInput};

use crate::{
    priority::{Priority, PriorityMut, PriorityStack},
    requirements::ActionRequirement,
};

pub trait ActionSource: Send + Sync + Sized {
    /// Where this action originates from.
    type Source: Send + Sync;
    /// Where this action originates from.
    fn source(&self) -> &Self::Source;
}
/// An action that must be put on an empty stack.
///
/// **NOTE:** if it implements `StackAction` in addition to this trait,
/// it can be put both on an empty stack and stacked stack.
pub trait IncitingAction<State, Input>: IncitingActionInfo<State> + ActionSource {
    /// Requirement must be satisfied before this action can be resolved.
    type Requirement: ActionRequirement<Priority<State>, Input>;

    fn resolve(
        self,
        priority: PriorityMut<Priority<State>>,
        input: <<Self::Requirement as ActionRequirement<
            Priority<State>,
            Input,
        >>::Filter as StateFilter<Priority<State>, Input>>::ValidOutput,
    ) -> Self::Resolved;
}
pub trait IncitingActionInfo<State> {
    /// The resolution of this inciting action.
    type Resolved;
    /// Can be stacked upon this inciting action.
    type Stackable;
}

/// An action that must be put on a stacked stack,
///
/// **NOTE:** if it implements `IncitingAction` in addition to this trait,
/// it can be put both on an empty stack and stacked stack.
pub trait StackAction<State, Input, IncitingAction: crate::actions::IncitingActionInfo<State>>:
    ActionSource
{
    /// Requirement must be satisfied before this action can be resolved.
    type Requirement: ActionRequirement<PriorityStack<State, IncitingAction>, Input>;

    /// The resolution of this action.
    type Resolved;
    fn resolve(
        self,
        priority: PriorityMut<PriorityStack<State, IncitingAction>>,
        input: <<Self::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
        >>::Filter as StateFilter<PriorityStack<State, IncitingAction>, Input>>::ValidOutput,
    ) -> Self::Resolved;
}
