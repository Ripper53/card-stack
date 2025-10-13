use state_validation::StateFilterInput;

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
    pub fn stack<IncitingAction: crate::actions::IncitingActionInfo<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityStack<State, IncitingAction> {
        PriorityStack::new(self, inciting_action)
    }
}
impl<State: GetState<InnerState>, InnerState> GetState<InnerState> for Priority<State> {
    fn state(&self) -> &InnerState {
        self.state.state()
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
impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
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
    fn stack<IncitingAction: crate::actions::IncitingActionInfo<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityMut<PriorityStack<State, IncitingAction>>;
}

impl<State> IncitingPriority<State> for PriorityMut<Priority<State>> {
    fn stack<IncitingAction: crate::actions::IncitingActionInfo<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityMut<PriorityStack<State, IncitingAction>> {
        PriorityMut::<PriorityStack<State, IncitingAction>>::new(PriorityStack::new(
            self.priority,
            inciting_action,
        ))
    }
}

pub trait StackPriority<State, IncitingAction: crate::actions::IncitingActionInfo<State>> {
    fn stack(self, action: impl Into<IncitingAction::Stackable>) -> Self;
}

pub struct PriorityStack<State, IncitingAction: crate::actions::IncitingActionInfo<State>> {
    priority: Priority<State>,
    stack: crate::Stack<State, IncitingAction>,
}

impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
    PriorityStack<State, IncitingAction>
{
    pub(crate) fn new(priority: Priority<State>, inciting_action: IncitingAction) -> Self {
        PriorityStack {
            priority,
            stack: crate::Stack::new(inciting_action),
        }
    }
}
impl<
    State: GetState<InnerState>,
    InnerState,
    IncitingAction: crate::actions::IncitingActionInfo<State>,
> GetState<InnerState> for PriorityStack<State, IncitingAction>
{
    fn state(&self) -> &InnerState {
        self.priority.state()
    }
}

pub trait Resolver<
    State,
    Input: StateFilterInput,
    IncitingAction: crate::actions::IncitingAction<State, Input>,
>: Sized
{
    fn resolve_next<
        R: IncitingResolver<State, Input, IncitingAction> + StackResolver<State, IncitingAction>,
    >(
        self,
    ) -> ResolveStack<Self, R::Resolved, R::HaltStack>;
}
impl<State, Input: StateFilterInput, IncitingAction: crate::actions::IncitingAction<State, Input>>
    Resolver<State, Input, IncitingAction> for PriorityStack<State, IncitingAction>
{
    fn resolve_next<
        R: IncitingResolver<State, Input, IncitingAction> + StackResolver<State, IncitingAction>,
    >(
        mut self,
    ) -> ResolveStack<Self, R::Resolved, R::HaltStack> {
        if let Some(action) = self.stack.pop() {
            match R::resolve_stack(
                PriorityMut::<PriorityStack<State, IncitingAction>>::new(self),
                action,
            ) {
                Resolve::Continue(priority) => ResolveStack::Next(priority),
                Resolve::Halt(data) => ResolveStack::Halt(data),
            }
        } else {
            let inciting_action = self.stack.take_inciting_action();
            ResolveStack::Complete(R::resolve_inciting(
                PriorityMut::<Priority<State>>::new(self.priority),
                inciting_action,
            ))
        }
    }
}
pub trait IncitingResolver<
    State,
    Input: StateFilterInput,
    IncitingAction: crate::actions::IncitingAction<State, Input>,
>
{
    type Resolved;
    fn resolve_inciting(
        prioriy: PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved;
}
pub trait StackResolver<State, IncitingAction: crate::actions::IncitingActionInfo<State>> {
    type HaltStack;
    fn resolve_stack(
        prioriy: PriorityMut<PriorityStack<State, IncitingAction>>,
        action: IncitingAction::Stackable,
    ) -> Resolve<PriorityStack<State, IncitingAction>, Self::HaltStack>;
}
pub enum Resolve<Priority, Halt> {
    Continue(Priority),
    Halt(Halt),
}
impl<State, Break, IncitingAction: crate::actions::IncitingActionInfo<State>>
    From<PriorityStack<State, IncitingAction>>
    for Resolve<PriorityStack<State, IncitingAction>, Break>
{
    fn from(priority: PriorityStack<State, IncitingAction>) -> Self {
        Resolve::Continue(priority)
    }
}
pub enum ResolveStack<Priority, Data, Halt> {
    Next(Priority),
    Complete(Data),
    Halt(Halt),
}
pub enum ResolveStackFully<Data, Broken> {
    Complete(Data),
    Broken(Broken),
}

impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
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
