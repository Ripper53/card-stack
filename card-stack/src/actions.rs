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
pub trait IncitingAction<State>: ActionSource {
    type EmptyStackRequirement: ActionRequirement<Priority<State>, Self>;
    /// Requirement must be satisfied before this action can be resolved.
    fn requirement(&self) -> Self::EmptyStackRequirement;

    type Stackable: crate::actions::StackAction<State, Self>;
    type ResolvedIncitingAction;
    fn resolve(self, priority: PriorityMut<Priority<State>>) -> Self::ResolvedIncitingAction;
}

/// An action that must be put on a stacked stack,
///
/// **NOTE:** if it implements `IncitingAction` in addition to this trait,
/// it can be put both on an empty stack and stacked stack.
pub trait StackAction<State, IncitingAction: crate::actions::IncitingAction<State>>:
    ActionSource
{
    type StackedRequirement: ActionRequirement<PriorityStack<State, IncitingAction>, Self>;
    /// Requirement must be satisfied before this action can be resolved.
    fn requirement(&self) -> Self::StackedRequirement;

    type ResolvedStackAction;
    fn resolve(
        self,
        priority: PriorityMut<PriorityStack<State, IncitingAction>>,
    ) -> Self::ResolvedStackAction;
}
