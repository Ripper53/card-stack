pub mod actions;
pub mod priority;
pub mod requirements;

pub struct Stack<State, IncitingAction: crate::actions::IncitingActionInfo<State>> {
    inciting_action: IncitingAction,
    stack: Vec<IncitingAction::Stackable>,
}

impl<State, IncitingAction: crate::actions::IncitingActionInfo<State> + std::fmt::Debug>
    std::fmt::Debug for Stack<State, IncitingAction>
where
    IncitingAction::Stackable: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field("inciting_action", &self.inciting_action)
            .field("stack", &self.stack)
            .finish()
    }
}

impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
    Stack<State, IncitingAction>
{
    pub(crate) fn new(inciting_action: IncitingAction) -> Self {
        Stack {
            inciting_action,
            stack: Vec::new(),
        }
    }
    pub fn inciting_action(&self) -> &IncitingAction {
        &self.inciting_action
    }
    pub fn full_stack(&self) -> &[IncitingAction::Stackable] {
        &self.stack
    }
    pub fn take_inciting_action(self) -> IncitingAction {
        self.inciting_action
    }
    pub fn stack(&mut self, stack: IncitingAction::Stackable) {
        self.stack.push(stack)
    }
    pub fn pop(&mut self) -> Option<IncitingAction::Stackable> {
        self.stack.pop()
    }
}

impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
    Stack<State, IncitingAction>
{
    pub fn into_state<NewState>(self) -> Stack<NewState, IncitingAction>
    where
        IncitingAction: crate::actions::IncitingActionInfo<NewState, Stackable = <IncitingAction as crate::actions::IncitingActionInfo<State>>::Stackable>,
    {
        Stack {
            stack: self.stack,
            inciting_action: self.inciting_action,
        }
    }
}

/// Marker trait which signifies a empty input.
/// Used to differentiate empty and non-empty inputs
/// for generic implementations using generic constraints.
pub trait EmptyInput {
    fn empty() -> Self;
}
impl EmptyInput for () {
    fn empty() -> Self {}
}
/// Marker trait which signifies a non-empty input.
/// Used to differentiate empty and non-empty inputs
/// for generic implementations using generic constraints.
pub trait NonEmptyInput {}
