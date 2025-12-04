use crate::{
    EmptyInput, NonEmptyInput,
    requirements::{RequirementAction, TryNewRequirementActionError},
};

pub trait GetState<State> {
    fn state(&self) -> &State;
}
pub trait TakeState<State> {
    type Remainder;
    fn take_state(self) -> (State, Self::Remainder);
}
pub trait CombineState<State> {
    type Combined;
    fn combine(self, state: State) -> Self::Combined;
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
        PriorityStack::new(self, inciting_action.into())
    }
}
impl<State: GetState<InnerState>, InnerState> GetState<InnerState> for Priority<State> {
    fn state(&self) -> &InnerState {
        self.state.state()
    }
}
#[derive(Debug)]
pub struct PriorityRemainder(());
impl<State> TakeState<State> for Priority<State> {
    type Remainder = PriorityRemainder;
    fn take_state(self) -> (State, Self::Remainder) {
        (self.state, PriorityRemainder(()))
    }
}
impl<State> CombineState<State> for PriorityRemainder {
    type Combined = Priority<State>;
    fn combine(self, state: State) -> Self::Combined {
        Priority::new(state)
    }
}

pub struct PriorityMut<Priority> {
    priority: Priority,
}
impl<State: GetState<InnerState>, InnerState> GetState<InnerState>
    for PriorityMut<Priority<State>>
{
    fn state(&self) -> &InnerState {
        self.priority.state.state()
    }
}
impl<
    State: GetState<InnerState>,
    InnerState,
    IncitingAction: crate::actions::IncitingActionInfo<State>,
> GetState<InnerState> for PriorityMut<PriorityStack<State, IncitingAction>>
{
    fn state(&self) -> &InnerState {
        self.priority.state()
    }
}
impl<State> PriorityMut<Priority<State>> {
    pub(crate) fn new(priority: Priority<State>) -> Self {
        PriorityMut { priority }
    }
    pub fn take_state(self) -> State {
        self.priority.state
    }
    pub fn take_priority(self) -> Priority<State> {
        self.priority
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
    pub fn stack(mut self, stack_action: impl Into<IncitingAction::Stackable>) -> Self {
        self.priority = self.priority.stack(stack_action.into());
        self
    }
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
        self.priority.state()
    }
}
pub struct PriorityStackRemainder<State, IncitingAction: crate::actions::IncitingActionInfo<State>>(
    crate::Stack<State, IncitingAction>,
);
impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>>
    PriorityStackRemainder<State, IncitingAction>
{
    pub fn create_stack<NewState>(self, state: NewState) -> PriorityStack<NewState, IncitingAction>
    where
        IncitingAction: crate::actions::IncitingActionInfo<NewState, Stackable = <IncitingAction as crate::actions::IncitingActionInfo<State>>::Stackable>,
    {
        PriorityStack {
            priority: Priority::new(state),
            stack: self.0.into_state(),
        }
    }
}

impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>> TakeState<State>
    for PriorityStack<State, IncitingAction>
{
    type Remainder = PriorityStackRemainder<State, IncitingAction>;
    fn take_state(self) -> (State, Self::Remainder) {
        (self.priority.state, PriorityStackRemainder(self.stack))
    }
}
impl<State, IncitingAction: crate::actions::IncitingActionInfo<State>> CombineState<State>
    for PriorityStackRemainder<State, IncitingAction>
{
    type Combined = PriorityStack<State, IncitingAction>;
    fn combine(self, state: State) -> Self::Combined {
        PriorityStack {
            priority: Priority::new(state),
            stack: self.0,
        }
    }
}

pub trait Resolver<State, Input, IncitingAction: crate::actions::IncitingAction<State, Input>>:
    Sized
{
    fn resolve_next<
        R: IncitingResolver<State, Input, IncitingAction> + StackResolver<State, IncitingAction>,
    >(
        self,
    ) -> ResolveStack<Self, R::Resolved, R::HaltStack>;
}
impl<State, Input, IncitingAction: crate::actions::IncitingAction<State, Input>>
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
    Input,
    IncitingAction: crate::actions::IncitingAction<State, Input>,
>
{
    type Resolved;
    fn resolve_inciting(
        prioriy: PriorityMut<Priority<State>>,
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
