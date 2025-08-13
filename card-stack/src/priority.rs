use crate::actions::StackAction;

pub trait GetState<State> {
    fn state(&self) -> &State;
}

/// `State`: state of the entire game.
pub struct Priority<State> {
    state: State,
}
impl<State> Priority<State> {
    pub fn new(state: State) -> Self {
        Priority { state }
    }
    pub fn stack<IncitingAction: crate::actions::IncitingAction<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityStack<State, IncitingAction> {
        PriorityStack::new(self, inciting_action)
    }
}
impl<State> GetState<State> for Priority<State> {
    fn state(&self) -> &State {
        &self.state
    }
}

pub struct PriorityMut<Priority> {
    priority: Priority,
}
impl<State> PriorityMut<Priority<State>> {
    pub(crate) fn new(priority: Priority<State>) -> Self {
        PriorityMut { priority }
    }
    pub fn take_priority(self) -> Priority<State> {
        self.priority
    }
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.priority.state
    }
}
impl<State, IncitingAction: crate::actions::IncitingAction<State>>
    PriorityMut<PriorityStack<State, IncitingAction>>
{
    pub(crate) fn new(priority: PriorityStack<State, IncitingAction>) -> Self {
        PriorityMut { priority }
    }
    pub fn take_priority(self) -> PriorityStack<State, IncitingAction> {
        self.priority
    }
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.priority.priority.state
    }
}

pub trait IncitingPriority<State> {
    fn stack<IncitingAction: crate::actions::IncitingAction<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityMut<PriorityStack<State, IncitingAction>>;
}

impl<State> IncitingPriority<State> for PriorityMut<Priority<State>> {
    fn stack<IncitingAction: crate::actions::IncitingAction<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityMut<PriorityStack<State, IncitingAction>> {
        PriorityMut::<PriorityStack<State, IncitingAction>>::new(PriorityStack::new(
            self.priority,
            inciting_action,
        ))
    }
}

pub trait StackPriority<State, IncitingAction: crate::actions::IncitingAction<State>> {
    fn stack(self, action: impl Into<IncitingAction::Stackable>) -> Self;
}

pub struct PriorityStack<State, IncitingAction: crate::actions::IncitingAction<State>> {
    priority: Priority<State>,
    stack: crate::Stack<State, IncitingAction>,
}

impl<State, IncitingAction: crate::actions::IncitingAction<State>>
    PriorityStack<State, IncitingAction>
{
    pub(crate) fn new(priority: Priority<State>, inciting_action: IncitingAction) -> Self {
        PriorityStack {
            priority,
            stack: crate::Stack::new(inciting_action),
        }
    }
}
impl<State, IncitingAction: crate::actions::IncitingAction<State>> GetState<State>
    for PriorityStack<State, IncitingAction>
{
    fn state(&self) -> &State {
        self.priority.state()
    }
}

impl<State, IncitingAction: crate::actions::IncitingAction<State>>
    PriorityStack<State, IncitingAction>
{
    pub fn resolve_next<R: Resolver<State, IncitingAction>>(
        mut self,
    ) -> ResolveStack<Self, R::Resolved> {
        if let Some(action) = self.stack.pop() {
            let r = action.resolve(PriorityMut::<PriorityStack<State, IncitingAction>>::new(
                self,
            ));
            match R::resolve_stack(r) {
                Resolve::Continue(priority) => {
                    self = priority;
                    ResolveStack::Next(self)
                }
                Resolve::Break(data) => ResolveStack::Complete(data),
            }
        } else {
            let r = self
                .stack
                .take_inciting_action()
                .resolve(PriorityMut::<Priority<State>>::new(self.priority));
            ResolveStack::Complete(R::resolve_inciting(r))
        }
    }
    pub fn resolve_fully<R: Resolver<State, IncitingAction>>(mut self) -> R::Resolved {
        while let Some(action) = self.stack.pop() {
            let r = action.resolve(PriorityMut::<PriorityStack<State, IncitingAction>>::new(
                self,
            ));
            match R::resolve_stack(r) {
                Resolve::Continue(priority) => self = priority,
                Resolve::Break(data) => return data,
            }
        }
        let r = self
            .stack
            .take_inciting_action()
            .resolve(PriorityMut::<Priority<State>>::new(self.priority));
        R::resolve_inciting(r)
    }
}
pub trait Resolver<State, IncitingAction: crate::actions::IncitingAction<State>>
where
    IncitingAction::Stackable: crate::actions::StackAction<State, IncitingAction>,
{
    type Resolved;
    fn resolve_inciting(action: IncitingAction::ResolvedIncitingAction) -> Self::Resolved;
    fn resolve_stack(
        action: <IncitingAction::Stackable as crate::actions::StackAction<
            State,
            IncitingAction,
        >>::ResolvedStackAction,
    ) -> Resolve<PriorityStack<State, IncitingAction>, Self::Resolved>;
}
pub enum Resolve<Priority, Break> {
    Continue(Priority),
    Break(Break),
}
pub enum ResolveStack<Priority, Data> {
    Next(Priority),
    Complete(Data),
}

impl<State, IncitingAction: crate::actions::IncitingAction<State>>
    StackPriority<State, IncitingAction> for PriorityMut<PriorityStack<State, IncitingAction>>
{
    fn stack(mut self, action: impl Into<IncitingAction::Stackable>) -> Self {
        self.priority.stack.stack(action.into());
        self
    }
}

#[derive(thiserror::Error)]
pub struct PriorityError<Priority, Error: std::error::Error + Send + Sync> {
    pub priority: Priority,
    #[source]
    pub error: Error,
}

impl<State, Error: std::error::Error + Send + Sync> std::fmt::Debug
    for PriorityError<State, Error>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.error)
    }
}
