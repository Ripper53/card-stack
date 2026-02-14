use crate::{
    EmptyInput, NonEmptyInput,
    requirements::{RequirementAction, TryNewRequirementActionError},
};

pub trait GetState<State> {
    fn state(&self) -> &State;
}

/// `State`: state of the entire game.
#[derive(Clone)]
pub struct Priority<State> {
    state: State,
}
impl<State: std::fmt::Debug> std::fmt::Debug for Priority<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Priority")
            .field("state", &self.state)
            .finish()
    }
}
impl<State> Priority<State> {
    pub fn new(state: State) -> Self {
        Priority { state }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn take_state(self) -> State {
        self.state
    }
    pub fn stack<IncitingAction: crate::actions::IncitingActionInfo<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityStack<State, IncitingAction> {
        PriorityStack::new(self, inciting_action.into())
    }
    pub fn into_state<NewState>(self) -> Priority<NewState>
    where
        State: Into<NewState>,
    {
        Priority::new(self.state.into())
    }
}
impl<State: GetState<InnerState>, InnerState> GetState<InnerState> for Priority<State> {
    fn state(&self) -> &InnerState {
        self.state.state()
    }
}

#[derive(Debug)]
pub struct PriorityMut<Priority> {
    priority: Priority,
}
impl<Priority> PriorityMut<Priority> {
    pub fn priority(&self) -> &Priority {
        &self.priority
    }
    pub fn priority_mut(&mut self) -> &mut Priority {
        &mut self.priority
    }
    pub fn take_priority(self) -> Priority {
        self.priority
    }
}
impl<State> PriorityMut<Priority<State>> {
    #[cfg(not(feature = "internals"))]
    pub(crate) fn new(priority: Priority<State>) -> Self {
        PriorityMut { priority }
    }
    #[cfg(feature = "internals")]
    pub fn new(priority: Priority<State>) -> Self {
        PriorityMut { priority }
    }
    pub fn take_state(self) -> State {
        self.priority.state
    }
    pub fn state(&self) -> &State {
        self.priority.state()
    }
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.priority.state
    }
    pub fn stack<IncitingAction: crate::actions::IncitingActionInfo<State>>(
        self,
        inciting_action: IncitingAction,
    ) -> PriorityMut<PriorityStack<State, IncitingAction>> {
        PriorityMut::<PriorityStack<State, IncitingAction>>::new(
            self.priority.stack(inciting_action),
        )
    }
    pub fn into_state<NewState>(self) -> PriorityMut<Priority<NewState>>
    where
        State: Into<NewState>,
    {
        PriorityMut::<Priority<_>>::new(self.priority.into_state())
    }
}
impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
    PriorityMut<PriorityStack<State, IncitingAction>>
{
    #[cfg(not(feature = "internals"))]
    pub(crate) fn new(priority: PriorityStack<State, IncitingAction>) -> Self {
        PriorityMut { priority }
    }
    #[cfg(feature = "internals")]
    pub fn new(priority: PriorityStack<State, IncitingAction>) -> Self {
        PriorityMut { priority }
    }
    pub fn state_mut(&mut self) -> &mut State {
        &mut self.priority.priority.state
    }
    pub fn stack(mut self, stack_action: impl Into<IncitingAction::Stackable>) -> Self {
        self.priority = self.priority.stack(stack_action.into());
        self
    }
    pub fn into_state<NewState, NewIncitingAction: crate::actions::IncitingActionInfo<NewState>>(
        self,
    ) -> PriorityMut<PriorityStack<NewState, NewIncitingAction>>
    where
        State: Into<NewState>,
        IncitingAction: Into<NewIncitingAction>,
        <IncitingAction as crate::actions::IncitingActionInfo<State>>::Stackable:
            Into<<NewIncitingAction as crate::actions::IncitingActionInfo<NewState>>::Stackable>,
    {
        PriorityMut::<PriorityStack<_, _>>::new(self.priority.into_state())
    }
}
impl<State: GetState<InnerState>, InnerState> GetState<InnerState> for PriorityMut<State> {
    fn state(&self) -> &InnerState {
        self.priority.state()
    }
}

pub struct PriorityStack<State, IncitingAction: crate::actions::IncitingActionInfo<State>> {
    priority: Priority<State>,
    stack: crate::Stack<State, IncitingAction>,
}
impl<State: Clone, IncitingAction: crate::actions::IncitingActionInfo<State> + Clone> Clone
    for PriorityStack<State, IncitingAction>
where
    IncitingAction::Stackable: Clone,
{
    fn clone(&self) -> Self {
        PriorityStack {
            priority: self.priority.clone(),
            stack: self.stack.clone(),
        }
    }
}
impl<OldState, OldIncitingAction: crate::actions::IncitingActionInfo<OldState>>
    PriorityStack<OldState, OldIncitingAction>
{
    pub fn into_state<NewState, NewIncitingAction: crate::actions::IncitingActionInfo<NewState>>(
        self,
    ) -> PriorityStack<NewState, NewIncitingAction>
    where
        OldState: Into<NewState>,
        OldIncitingAction: Into<NewIncitingAction>,
        <OldIncitingAction as crate::actions::IncitingActionInfo<OldState>>::Stackable:
            Into<<NewIncitingAction as crate::actions::IncitingActionInfo<NewState>>::Stackable>,
    {
        PriorityStack {
            priority: self.priority.into_state(),
            stack: self.stack.into_state(),
        }
    }
}
impl<
    State: std::fmt::Debug,
    IncitingAction: crate::actions::IncitingActionInfo<State> + std::fmt::Debug,
> std::fmt::Debug for PriorityStack<State, IncitingAction>
where
    IncitingAction::Stackable: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PriorityStack")
            .field("priority", &self.priority)
            .field("stack", &self.stack)
            .finish()
    }
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
    #[cfg(feature = "internals")]
    pub fn from_stack(
        priority: Priority<State>,
        stack: crate::Stack<State, IncitingAction>,
    ) -> Self {
        PriorityStack { priority, stack }
    }
    #[cfg(feature = "internals")]
    pub fn take_contents(self) -> (State, crate::Stack<State, IncitingAction>) {
        (self.priority.state, self.stack)
    }
    pub fn priority(&self) -> &Priority<State> {
        &self.priority
    }
    pub fn state(&self) -> &State {
        self.priority.state()
    }
    /// Does NOT include the inciting action.
    pub fn stack_count(&self) -> usize {
        self.stack.stack.len()
    }
    pub fn stack(
        mut self,
        stack_action: impl Into<IncitingAction::Stackable>,
    ) -> PriorityStack<State, IncitingAction> {
        self.stack.stack(stack_action.into());
        self
    }
}
impl<
    State: GetState<InnerState>,
    InnerState,
    IncitingAction: crate::actions::IncitingActionInfo<State>,
> GetState<InnerState> for PriorityStack<State, IncitingAction>
{
    fn state(&self) -> &InnerState {
        self.priority.state.state()
    }
}

pub trait Resolver<State, Input, IncitingAction: crate::actions::IncitingAction<State, Input>>:
    Sized
{
    fn resolve_next<
        R: IncitingResolver<State, Input, IncitingAction> + StackResolver<State, IncitingAction>,
    >(
        self,
        resolver: &mut R,
    ) -> ResolveStack<Self, R::Resolved, R::HaltStack>;
}
impl<State, Input, IncitingAction: crate::actions::IncitingAction<State, Input>>
    Resolver<State, Input, IncitingAction> for PriorityStack<State, IncitingAction>
{
    fn resolve_next<
        R: IncitingResolver<State, Input, IncitingAction> + StackResolver<State, IncitingAction>,
    >(
        mut self,
        resolver: &mut R,
    ) -> ResolveStack<Self, R::Resolved, R::HaltStack> {
        if let Some(action) = self.stack.pop() {
            match resolver.resolve_stack(
                PriorityMut::<PriorityStack<State, IncitingAction>>::new(self),
                action,
            ) {
                Resolve::Continue(priority) => ResolveStack::Next(priority),
                Resolve::Halt(data) => ResolveStack::Halt(data),
            }
        } else {
            let inciting_action = self.stack.take_inciting_action();
            ResolveStack::Complete(resolver.resolve_inciting(
                PriorityMut::<Priority<State>>::new(self.priority),
                inciting_action,
            ))
        }
    }
}
pub trait IncitingResolver<
    State,
    Input,
    IncitingAction: crate::actions::IncitingAction<State, Input>,
>
{
    type Resolved;
    fn resolve_inciting(
        &mut self,
        priority: PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved;
}
impl<
    State,
    IncitingAction: crate::actions::IncitingAction<State, (), Requirement = ()>,
    Resolver: StackResolver<State, IncitingAction>,
> IncitingResolver<State, (), IncitingAction> for Resolver
{
    type Resolved = IncitingAction::Resolved;
    fn resolve_inciting(
        &mut self,
        priority: PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved {
        action.resolve(priority, ())
    }
}
impl<
    State,
    Input: NonEmptyInput,
    IncitingAction: crate::actions::IncitingAction<State, Input>,
    Resolver: StackResolver<State, IncitingAction>,
> IncitingResolver<State, Input, IncitingAction> for Resolver
{
    type Resolved = Result<
        RequirementAction<Priority<State>, Input, IncitingAction>,
        TryNewRequirementActionError<Priority<State>, IncitingAction>,
    >;
    fn resolve_inciting(
        &mut self,
        priority: PriorityMut<Priority<State>>,
        action: IncitingAction,
    ) -> Self::Resolved {
        RequirementAction::<Priority<State>, Input, IncitingAction>::try_new(
            priority.take_priority(),
            action,
        )
    }
}
pub trait StackResolver<State, IncitingAction: crate::actions::IncitingActionInfo<State>> {
    type HaltStack;
    fn resolve_stack(
        &mut self,
        priority: PriorityMut<PriorityStack<State, IncitingAction>>,
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
