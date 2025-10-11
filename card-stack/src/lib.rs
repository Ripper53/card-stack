pub mod actions;
pub mod priority;
pub mod requirements;

pub struct Stack<State, IncitingAction: crate::actions::IncitingStackable<State>> {
    inciting_action: IncitingAction,
    stack: Vec<IncitingAction::Stackable>,
}

impl<State, IncitingAction: crate::actions::IncitingStackable<State>> Stack<State, IncitingAction> {
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
