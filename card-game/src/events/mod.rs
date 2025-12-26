use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::{create_valid_identification, identifications::SourceCardID};
use card_stack::{
    actions::{ActionSource, IncitingAction, IncitingActionInfo, StackAction},
    priority::{GetState, IncitingResolver, Priority, PriorityMut, PriorityStack},
};
use state_validation::{
    Condition, StateFilter, ValidAction,
    dynamic::{DynStateFilter, DynValidAction, DynValidActionExecutionError},
};

pub struct EventManager<State: 'static, Ev: Event<PriorityMut<State>>, Output> {
    events: Vec<DynEventListener<State, Ev, Output>>,
}
pub type EventPriorityStack<State, Ev: Event<PriorityMut<State>>, Output> =
    PriorityStack<State, EventAction<Priority<State>, Ev, Output>>;
pub(crate) struct DynEventListener<State, Ev: Event<PriorityMut<State>>, Output> {
    valid_action: Box<dyn AnyClone>,
    filter: for<'a> fn(&'a State, Box<dyn Any>) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
    get_action:
        for<'a> fn(Box<dyn Any>, &'a State, &'a Ev, Box<dyn Any>) -> EventAction<State, Ev, Output>,
}
impl<State, Ev: Event<PriorityMut<State>>, Output: 'static> DynEventListener<State, Ev, Output> {
    fn new<T: EventListener<State, Ev> + 'static>(valid_action: T) -> Self
    where
        T::Action: 'static,
        T::ActionInput: 'static,
        <T::Filter as StateFilter<State, T>>::ValidOutput: 'static,
        <T::Filter as StateFilter<State, T>>::Error: 'static,
        <T::Action as ValidAction<PriorityMut<State>, T::ActionInput>>::Output: Into<Output>,
        <<T::Action as ValidAction<PriorityMut<State>, T::ActionInput>>::Filter as StateFilter<
            PriorityMut<State>,
            T::ActionInput,
        >>::ValidOutput: 'static,
        <<T::Action as ValidAction<PriorityMut<State>, T::ActionInput>>::Filter as StateFilter<
            PriorityMut<State>,
            T::ActionInput,
        >>::Error: 'static,
    {
        DynEventListener {
            valid_action: Box::new(valid_action),
            filter: |state, input| match <T::Filter>::filter(state, *input.downcast().unwrap()) {
                Ok(v) => Ok(Box::new(v)),
                Err(e) => Err(Box::new(e)),
            },
            get_action: |valid_action, state, event, valid| {
                EventAction::new::<T>(
                    state,
                    valid_action.downcast_ref().unwrap(),
                    event,
                    *valid.downcast().unwrap(),
                )
            },
        }
    }
    fn get_action(
        &self,
        state: &State,
        event: &Ev,
    ) -> Result<EventAction<State, Ev, Output>, Box<dyn std::error::Error>> {
        match (self.filter)(state, self.valid_action.any_clone()) {
            Ok(valid) => Ok((self.get_action)(
                self.valid_action.any_clone(),
                state,
                event,
                valid,
            )),
            Err(error) => Err(error),
        }
    }
}
impl<State, Ev: Event<PriorityMut<State>>, Output> Clone for DynEventListener<State, Ev, Output> {
    fn clone(&self) -> Self {
        DynEventListener {
            valid_action: self.valid_action.any_clone(),
            filter: self.filter,
            get_action: self.get_action,
        }
    }
}
pub(crate) trait AnyClone: Any {
    fn any_clone(&self) -> Box<dyn AnyClone>;
}
impl<T: Clone + 'static> AnyClone for T {
    fn any_clone(&self) -> Box<dyn AnyClone> {
        Box::new(self.clone())
    }
}
impl dyn AnyClone {
    pub(crate) fn any_clone_duplication(&self) -> Box<dyn AnyClone> {
        self.any_clone()
    }
}

impl<State: 'static, Ev: Event<PriorityMut<State>>, Output> std::fmt::Debug
    for EventManager<State, Ev, Output>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventManager").finish_non_exhaustive()
    }
}
impl<State: 'static, Ev: Event<PriorityMut<State>>, Output> Clone
    for EventManager<State, Ev, Output>
{
    fn clone(&self) -> Self {
        EventManager {
            events: self.events.clone(),
        }
    }
}
impl<State: 'static, Ev: Event<PriorityMut<State>>, Output> Default
    for EventManager<State, Ev, Output>
{
    fn default() -> Self {
        Self::empty()
    }
}
impl<State: 'static, Ev: Event<PriorityMut<State>>, Output> EventManager<State, Ev, Output> {
    pub fn empty() -> Self {
        EventManager { events: Vec::new() }
    }
    pub fn count(&self) -> usize {
        self.events.len()
    }
}
impl<EventState: 'static, Ev: Event<PriorityMut<EventState>>, Output: 'static>
    EventManager<EventState, Ev, Output>
{
    pub(crate) fn new(events: Vec<DynEventListener<EventState, Ev, Output>>) -> Self {
        EventManager { events }
    }
    pub fn add_listener<Listener: EventListener<EventState, Ev>>(&mut self, listener: Listener)
    where
        <Listener::Action as ValidAction<PriorityMut<EventState>, Listener::ActionInput>>::Output:
            Into<Output>,
    {
        let listener = DynEventListener::new(listener);
        self.events.push(listener);
    }
}
impl<EventState: 'static, Ev: Event<PriorityMut<EventState>>, Output: 'static>
    EventManager<EventState, Ev, Output>
{
    fn collect_actions(
        &self,
        state: &PriorityMut<EventState>,
        event: Ev,
    ) -> CollectedActions<EventState, Ev, Output> {
        let actions = self
            .events
            .iter()
            .filter_map(|listener| {
                if let Ok(event_action) = listener.get_action(state.priority(), &event)
                    && event_action
                        .action
                        .filter()
                        .filter(&state, event_action.event_input.any_clone())
                        .is_ok()
                {
                    Some(event_action)
                } else {
                    None
                }
            })
            .collect();
        CollectedActions { event, actions }
    }
}
struct CollectedActions<State, Ev: Event<PriorityMut<State>>, Output> {
    event: Ev,
    actions: Vec<EventAction<State, Ev, Output>>,
}
impl<EventState, Ev: Event<PriorityMut<EventState>>, Output>
    CollectedActions<EventState, Ev, Output>
{
    fn simultaneous_action_manager(
        self,
        state: EventState,
    ) -> SimultaneousActionManager<EventState, Ev, Output>
//where
        //EventAction<EventState, Ev, Output>: Into<Ev::Stackable>,
    {
        SimultaneousActionManager {
            state,
            event: self.event,
            actions: self
                .actions
                .into_iter()
                .enumerate()
                .map(|(i, action)| (SimultaneousActionID(i), action))
                .collect(),
        }
    }
}
pub struct SimultaneousActionManager<State, Ev: Event<PriorityMut<State>>, Output> {
    state: State,
    event: Ev,
    actions: HashMap<SimultaneousActionID, EventAction<State, Ev, Output>>,
}
impl<State: std::fmt::Debug, Ev: Event<PriorityMut<State>>, Output> std::fmt::Debug
    for SimultaneousActionManager<State, Ev, Output>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimultaneousActionManager")
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}
pub struct SingleAction<State, Ev: Event<PriorityMut<State>>, Output> {
    state: State,
    event_action: EventAction<State, Ev, Output>,
}
impl<State, Ev: Event<PriorityMut<Priority<State>>>, Output>
    SingleAction<Priority<State>, Ev, Output>
{
    pub fn resolve(self) -> PriorityStack<State, EventAction<Priority<State>, Ev, Output>>
    where
        EventAction<Priority<State>, Ev, Output>: IncitingAction<State, ()>,
    {
        self.state.stack(self.event_action)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct SimultaneousActionID(usize);
use crate as card_game;
create_valid_identification!(ValidSimultaneousActionID, SimultaneousActionID, with_copy);
impl<F> ValidSimultaneousActionID<F> {
    pub(crate) fn new(id: SimultaneousActionID) -> Self {
        ValidSimultaneousActionID(id, std::marker::PhantomData::default())
    }
}
impl<State, Ev: Event<PriorityMut<State>>, Output> SimultaneousActionManager<State, Ev, Output>
//where
//EventAction<State, Ev, Output>: Into<Ev::Stackable>,
{
    pub fn verify(&self, id: SimultaneousActionID) -> Option<ValidSimultaneousActionID<()>> {
        if self.actions.contains_key(&id) {
            Some(ValidSimultaneousActionID::new(id))
        } else {
            None
        }
    }
    pub fn simultaneous_action_count(&self) -> usize {
        self.actions.len()
    }
    pub fn simultaneous_action_ids(&self) -> impl Iterator<Item = ValidSimultaneousActionID<()>> {
        self.actions
            .keys()
            .copied()
            .map(|id| ValidSimultaneousActionID::new(id))
    }
    pub fn unresolved_simultaneous_action_ids(
        &self,
    ) -> impl Iterator<Item = ValidSimultaneousActionID<()>> {
        self.actions
            .iter()
            .enumerate()
            .map(|(index, action)| ValidSimultaneousActionID::new(SimultaneousActionID(index)))
    }
}
impl<State: Clone, Ev: Event<PriorityMut<State>>, Output> Clone
    for SimultaneousActionManager<State, Ev, Output>
{
    fn clone(&self) -> Self {
        SimultaneousActionManager {
            state: self.state.clone(),
            event: self.event.clone(),
            actions: self.actions.clone(),
        }
    }
}
/*impl<State: GetEventManager<Ev>, Ev: Event<State> + Event<State::State>, Output>
    SimultaneousActionManager<State, Ev, Output>
where
    <Ev as Event<State>>::Input: 'static,
    <Ev as Event<State::State>>::Input: 'static,
    EventAction<State, Ev, Output>: Into<<Ev as Event<State>>::Stackable>,
    EventAction<State::State, Ev, Output>: IncitingActionInfo<State>
        + Into<<Ev as Event<State::State>>::Stackable>
        + Into<<EventAction<State::State, Ev, Output> as IncitingActionInfo<State>>::Stackable>,
{
    /// Resolves in order.
    /// First element is put on the stack last.
    pub fn resolve(
        mut self,
        mut order: Vec<ValidSimultaneousActionID<()>>,
    ) -> Result<
        PriorityStack<State, EventAction<State::State, Output>>,
        ResolveSimultaneousActionsError<State, Ev, Output>,
    > {
        order.dedup();
        if order.len() != self.actions.len()
            || !self
                .actions
                .keys()
                .copied()
                .all(|v| order.contains(&ValidSimultaneousActionID::new(v)))
        {
            return Err(ResolveSimultaneousActionsError::NotAllActionsAreOrdered(
                self,
            ));
        }
        let inciting_action_id = order.pop().unwrap();
        let action = self.actions.remove(&inciting_action_id.0).unwrap();
        let mut stack =
            Priority::new(self.state).stack(EventAction::new(self.event_input.clone(), action));
        for action_id in order.iter().rev() {
            let action = self.actions.remove(&action_id.0).unwrap();
            stack = stack.stack(EventAction::new(self.event_input.clone(), action));
        }
        Ok(stack)
    }
    pub fn execute(
        mut self,
        action_id: ValidSimultaneousActionID<()>,
    ) -> <Output as CombineState<<State as TakeState<State::State>>::Remainder>>::Combined
    where
        State: TakeState<State::State>,
        Output: CombineState<<State as TakeState<State::State>>::Remainder>,
        DynAction<<State as GetEventManager<Ev>>::State, Ev, Output>: ValidAction<
                State::State,
                <Ev as Event<<State as GetEventManager<Ev>>::State>>::Input,
                Filter = (),
                Output = Result<Output, DynStateError<State::State>>,
            >,
    {
        let (state, remainder) = self.state.take_state();
        let action = self.actions.remove(&action_id.0).unwrap();
        if let Ok(output) = action.with_valid_input(state, self.event_input) {
            output.combine(remainder)
        } else {
            unreachable!()
        }
    }
}*/
impl<
    State: GetStackEventManager<Ev, EventAction<Priority<State>, Ev, Output>> + 'static,
    Ev: Event<PriorityMut<Priority<State>>>
        + Event<
            PriorityStack<State, EventAction<Priority<State>, Ev, Output>>,
            Stackable = <EventAction<Priority<State>, Ev, Output> as IncitingActionInfo<State>>::Stackable,
        > + Event<
            PriorityMut<PriorityStack<State, EventAction<Priority<State>, Ev, Output>>>,
            Stackable = <EventAction<Priority<State>, Ev, Output> as IncitingActionInfo<State>>::Stackable,
        >,
    Output: 'static,
> SimultaneousActionManager<Priority<State>, Ev, Output>
where
    EventAction<PriorityStack<State, EventAction<Priority<State>, Ev, Output>>, Ev, State::Output>:
        Into<
            <EventAction<Priority<State>, Ev, Output> as IncitingActionInfo<State>>::Stackable,
        >,
{
    pub fn stack_inciting(
        mut self,
        inciting_action_id: ValidSimultaneousActionID<()>,
    ) -> TriggeredStackEventResolution<State, Ev, EventAction<Priority<State>, Ev, Output>> {
        let action = self.actions.remove(&inciting_action_id.0).unwrap();
        let mut stack = self
            .state
            .stack(action);
        TriggeredEvent::<PriorityStack<_, _>, _>::new(stack, self.event).collect()
    }
}
impl<
    State,
    Ev: Event<PriorityMut<PriorityStack<State, IncitingAction>>>,
    IncitingAction: IncitingActionInfo<State> + 'static,
    Output,
> SimultaneousActionManager<PriorityStack<State, IncitingAction>, Ev, Output>
where
    EventAction<PriorityStack<State, IncitingAction>, Ev, Output>: Into<IncitingAction::Stackable>,
{
    /// Resolves in order.
    /// First element is put on the stack last.
    pub fn resolve(
        mut self,
        mut order: Vec<ValidSimultaneousActionID<()>>,
    ) -> Result<
        PriorityStack<State, IncitingAction>,
        ResolveSimultaneousActionsError<PriorityStack<State, IncitingAction>, Ev, Output>,
    > {
        order.dedup();
        if order.len() != self.actions.len()
            || !self
                .actions
                .keys()
                .copied()
                .all(|v| order.contains(&ValidSimultaneousActionID::new(v)))
        {
            return Err(ResolveSimultaneousActionsError::NotAllActionsAreOrdered(
                self,
            ));
        }
        let inciting_action_id = order.pop().unwrap();
        let action = self.actions.remove(&inciting_action_id.0).unwrap();
        let mut stack = self.state.stack(action);
        for action_id in order.iter().rev() {
            let action = self.actions.remove(&action_id.0).unwrap();
            stack = stack.stack(action);
        }
        Ok(stack)
    }
}
pub struct EventAction<EventState, Ev: Event<PriorityMut<EventState>>, Output> {
    event_input: Box<dyn AnyClone>,
    action: DynValidAction<PriorityMut<EventState>, Box<dyn Any>, Output>,
    _m: std::marker::PhantomData<Ev>,
}
impl<EventState, Ev: Event<PriorityMut<EventState>>, Output> std::fmt::Debug
    for EventAction<EventState, Ev, Output>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventAction").finish_non_exhaustive()
    }
}
impl<EventState, Ev: Event<PriorityMut<EventState>>, Output> Clone
    for EventAction<EventState, Ev, Output>
{
    fn clone(&self) -> Self {
        EventAction {
            event_input: self.event_input.any_clone(),
            action: self.action.clone(),
            _m: std::marker::PhantomData::default(),
        }
    }
}
impl<EventState, Ev: Event<PriorityMut<EventState>>, Output: 'static>
    EventAction<EventState, Ev, Output>
{
    fn new<T: EventListener<EventState, Ev>>(
        state: &EventState,
        event_listener: &T,
        event: &Ev,
        valid: <T::Filter as StateFilter<EventState, T>>::ValidOutput,
    ) -> Self
    where
        T::Action: 'static,
        T::ActionInput: 'static,
        <T::Filter as StateFilter<EventState, T>>::ValidOutput: 'static,
        <T::Filter as StateFilter<EventState, T>>::Error: 'static,
        <T::Action as ValidAction<PriorityMut<EventState>, T::ActionInput>>::Output: Into<Output>,
        <<T::Action as ValidAction<PriorityMut<EventState>, T::ActionInput>>::Filter as StateFilter<
            PriorityMut<EventState>,
            T::ActionInput,
        >>::ValidOutput: 'static,
        <<T::Action as ValidAction<PriorityMut<EventState>, T::ActionInput>>::Filter as StateFilter<
            PriorityMut<EventState>,
            T::ActionInput,
        >>::Error: 'static,
    {
        struct IntoIndirection<T, Input, Output>(T, std::marker::PhantomData<(Input, Output)>);
        impl<T: Clone, Input, Output> Clone for IntoIndirection<T, Input, Output> {
            fn clone(&self) -> Self {
                IntoIndirection(self.0.clone(), std::marker::PhantomData::default())
            }
        }
        struct AnyCastFilter<T>(std::marker::PhantomData<T>);
        impl<State, T: 'static> StateFilter<State, Box<dyn Any>> for AnyCastFilter<T> {
            type ValidOutput = T;
            type Error = std::convert::Infallible;
            fn filter(_: &State, value: Box<dyn Any>) -> Result<Self::ValidOutput, Self::Error> {
                Ok(*value.downcast().unwrap())
            }
        }
        impl<State, Input: 'static, T: ValidAction<State, Input>, Output>
            ValidAction<State, Box<dyn Any>> for IntoIndirection<T, Input, Output>
        where
            T::Output: Into<Output>,
        {
            type Filter = (
                Condition<Box<dyn Any>, AnyCastFilter<Input>>,
                Condition<Input, T::Filter>,
            );
            type Output = Output;
            fn with_valid_input(
                self,
                state: State,
                valid: <Self::Filter as StateFilter<State, Box<dyn Any>>>::ValidOutput,
            ) -> Self::Output {
                self.0.with_valid_input(state, valid).into()
            }
        }
        let (action, event_input) = T::action(event_listener, state, event, valid);
        EventAction {
            event_input: Box::new(event_input),
            action: DynValidAction::new(IntoIndirection(
                action,
                std::marker::PhantomData::default(),
            )),
            _m: std::marker::PhantomData::default(),
        }
    }
}
impl<EventState, Ev: Event<PriorityMut<Priority<EventState>>>, Output>
    IncitingAction<EventState, ()> for EventAction<Priority<EventState>, Ev, Output>
{
    type Requirement = ();
    fn resolve(
        self,
        priority: PriorityMut<Priority<EventState>>,
        (): <<Self::Requirement as card_stack::requirements::ActionRequirement<
            Priority<EventState>,
            (),
        >>::Filter as StateFilter<Priority<EventState>, ()>>::ValidOutput,
    ) -> Self::Resolved {
        match self.action.with_valid_input(priority, self.event_input) {
            Ok(output) => EventActionResolution::Resolved(output),
            Err(e) => EventActionResolution::Fizzled {
                state: e.state,
                error: e.error,
            },
        }
    }
}
impl<EventState, Ev: Event<PriorityMut<Priority<EventState>>>, Output>
    IncitingActionInfo<EventState> for EventAction<Priority<EventState>, Ev, Output>
{
    type Resolved = EventActionResolution<PriorityMut<Priority<EventState>>, Output>;
    type Stackable = Ev::Stackable;
}
impl<
    EventState,
    Ev: Event<PriorityMut<PriorityStack<EventState, T>>>,
    Output,
    T: IncitingActionInfo<EventState>,
> IncitingActionInfo<EventState> for EventAction<PriorityStack<EventState, T>, Ev, Output>
{
    type Resolved = EventActionResolution<PriorityMut<PriorityStack<EventState, T>>, Output>;
    type Stackable = Ev::Stackable;
}
impl<
    T: IncitingActionInfo<EventState>,
    //+ IncitingActionInfo<Output, Stackable = <T as IncitingActionInfo<EventState>>::Stackable>,
    EventState,
    Ev: Event<PriorityMut<PriorityStack<EventState, T>>>,
    Output,
> StackAction<EventState, (), T> for EventAction<PriorityStack<EventState, T>, Ev, Output>
{
    type Requirement = ();
    type Resolved = EventActionResolution<PriorityMut<PriorityStack<EventState, T>>, Output>;
    fn resolve(
        self,
        priority: PriorityMut<PriorityStack<EventState, T>>,
        (): <<Self::Requirement as card_stack::requirements::ActionRequirement<
            PriorityStack<EventState, T>,
            (),
        >>::Filter as StateFilter<PriorityStack<EventState, T>, ()>>::ValidOutput,
    ) -> Self::Resolved {
        //let (state, stack) = priority.take_priority().take_contents();
        match self.action.with_valid_input(priority, self.event_input) {
            Ok(output) => EventActionResolution::Resolved(output),
            Err(e) => EventActionResolution::Fizzled {
                state: e.state,
                error: e.error,
            },
        }
    }
}
pub enum EventActionResolution<State, Output> {
    Resolved(Output),
    Fizzled {
        state: State,
        /// Why did it fizzle?
        error: Box<dyn std::error::Error>,
    },
}
#[derive(thiserror::Error)]
pub enum ResolveSimultaneousActionsError<State, Ev: Event<PriorityMut<State>>, Output> {
    #[error("not all actions are ordered")]
    NotAllActionsAreOrdered(SimultaneousActionManager<State, Ev, Output>),
}
impl<State: std::fmt::Debug, Ev: Event<PriorityMut<State>>, Output> std::fmt::Debug
    for ResolveSimultaneousActionsError<State, Ev, Output>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveSimultaneousActionsError::NotAllActionsAreOrdered(action_manager) => f
                .debug_tuple("NotAllActionsAreOrdered")
                .field(action_manager)
                .finish(),
        }
    }
}

pub struct TriggeredEvent<State, Ev: Event<PriorityMut<State>>> {
    state: State,
    event: Ev,
}

impl<State: std::fmt::Debug, Ev: Event<PriorityMut<State>> + std::fmt::Debug> std::fmt::Debug
    for TriggeredEvent<State, Ev>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TriggeredEvent")
            .field("state", &self.state)
            .field("event", &self.event)
            .finish()
    }
}

impl<State, Ev: Event<PriorityMut<State>>> TriggeredEvent<State, Ev> {
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn event(&self) -> &Ev {
        &self.event
    }
}
impl<State: GetEventManager<Ev> + 'static, Ev: Event<PriorityMut<Priority<State>>>>
    TriggeredEvent<Priority<State>, Ev>
//where
//EventAction<Priority<State>, Ev, State::Output>:
//Into<<Ev as Event<PriorityMut<Priority<State>>>>::Stackable>,
{
    pub fn new(state: Priority<State>, event: Ev) -> Self {
        TriggeredEvent { state, event }
    }
    pub fn collect(self) -> TriggeredEventResolution<State, Ev> {
        let event_manager = self.state.state().event_manager();
        // TODO: I don't like the fact that we are instantiating a priority mut!
        let priority_mut = PriorityMut::<Priority<_>>::new(self.state);
        let simultaneous_action_manager = event_manager
            .collect_actions(&priority_mut, self.event)
            .simultaneous_action_manager(priority_mut.take_priority());
        match simultaneous_action_manager.simultaneous_action_count() {
            0 => TriggeredEventResolution::None(simultaneous_action_manager.state.take_state()),
            1 => {
                let event_action = simultaneous_action_manager
                    .actions
                    .into_iter()
                    .next()
                    .unwrap()
                    .1;
                TriggeredEventResolution::Action(
                    simultaneous_action_manager.state.stack(event_action),
                )
            }
            _ => TriggeredEventResolution::SimultaneousActions(simultaneous_action_manager),
        }
    }
}
pub enum TriggeredEventResolution<
    State: GetEventManager<Ev>,
    Ev: Event<PriorityMut<Priority<State>>>,
>
//where
//EventAction<Priority<State>, Ev, State::Output>: Into<Ev::Stackable>,
{
    None(State),
    Action(PriorityStack<State, EventAction<Priority<State>, Ev, State::Output>>),
    SimultaneousActions(SimultaneousActionManager<Priority<State>, Ev, State::Output>),
}
impl<
    State: GetStackEventManager<Ev, IncitingAction> + 'static,
    Ev: Event<PriorityStack<State, IncitingAction>>
        + Event<
            PriorityMut<PriorityStack<State, IncitingAction>>,
            Stackable = <Ev as Event<PriorityStack<State, IncitingAction>>>::Stackable,
        >,
    IncitingAction: IncitingActionInfo<State> + 'static,
> TriggeredEvent<PriorityStack<State, IncitingAction>, Ev>
where
    EventAction<PriorityStack<State, IncitingAction>, Ev, State::Output>:
        Into<<Ev as Event<PriorityStack<State, IncitingAction>>>::Stackable>,
{
    pub fn new(state: PriorityStack<State, IncitingAction>, event: Ev) -> Self {
        TriggeredEvent { state, event }
    }
    pub fn collect(self) -> TriggeredStackEventResolution<State, Ev, IncitingAction>
    where
        EventAction<PriorityStack<State, IncitingAction>, Ev, State::Output>:
            Into<IncitingAction::Stackable>,
    {
        let event_manager = self.state.state().stack_event_manager();
        // TODO: I don't like the fact that we are instantiating a priority mut!
        let priority_mut = PriorityMut::<PriorityStack<_, _>>::new(self.state);
        let mut simultaneous_action_manager = event_manager
            .collect_actions(&priority_mut, self.event)
            .simultaneous_action_manager(priority_mut.take_priority());
        match simultaneous_action_manager.simultaneous_action_count() {
            0 => TriggeredStackEventResolution::None(simultaneous_action_manager.state),
            1 => {
                let event_action = simultaneous_action_manager
                    .actions
                    .into_iter()
                    .next()
                    .unwrap()
                    .1;
                TriggeredStackEventResolution::Action(
                    simultaneous_action_manager.state.stack(event_action),
                )
            }
            _ => TriggeredStackEventResolution::SimultaneousActions(simultaneous_action_manager),
        }
    }
}
pub enum TriggeredStackEventResolution<
    State: GetStackEventManager<Ev, IncitingAction>,
    Ev: Event<PriorityMut<PriorityStack<State, IncitingAction>>>,
    IncitingAction: IncitingActionInfo<State>,
> where
    EventAction<PriorityStack<State, IncitingAction>, Ev, State::Output>: Into<Ev::Stackable>,
{
    /// No actions were added to the stack.
    None(PriorityStack<State, IncitingAction>),
    /// A single action was added to the stack.
    Action(PriorityStack<State, IncitingAction>),
    /// Multiple actions need to be added to the stack at once, handle such a case.
    SimultaneousActions(
        SimultaneousActionManager<PriorityStack<State, IncitingAction>, Ev, State::Output>,
    ),
}

pub trait GetEventManager<Ev: Event<PriorityMut<Priority<Self>>>>: Sized {
    type Output;
    fn event_manager(&self) -> EventManager<Priority<Self>, Ev, Self::Output>;
}

pub trait GetStackEventManager<
    Ev: Event<PriorityMut<PriorityStack<Self, IncitingAction>>>,
    IncitingAction: IncitingActionInfo<Self>,
>: Sized
{
    type Output;
    fn stack_event_manager(
        &self,
    ) -> EventManager<PriorityStack<Self, IncitingAction>, Ev, Self::Output>;
}

pub trait AddEventListener<State, Ev: Event<PriorityMut<State>>> {
    type Output;
    fn add_listener<Listener: EventListener<State, Ev>>(&mut self, listener: Listener)
    where
        <Listener::Action as ValidAction<PriorityMut<State>, Listener::ActionInput>>::Output:
            Into<Self::Output>;
}

pub trait Event<State>: Clone + 'static {
    type Stackable;
    //fn event_id() -> EventID;
}
impl<State, T: Event<State>> Event<Priority<State>> for T {
    type Stackable = T::Stackable;
}
impl<State, T: Event<State>> Event<PriorityMut<Priority<State>>> for T {
    type Stackable = T::Stackable;
}
impl<
    State,
    IncitingAction: IncitingActionInfo<State>,
    T: Event<PriorityStack<State, IncitingAction>>,
> Event<PriorityMut<PriorityStack<State, IncitingAction>>> for T
{
    type Stackable = T::Stackable;
}

pub trait EventListenerConstructor<State, Ev: Event<PriorityMut<State>>>:
    EventListener<State, Ev>
{
    type Input;
    fn new_listener(source_card_id: SourceCardID, input: Self::Input) -> Self;
}
pub trait EventListener<State, Ev: Event<PriorityMut<State>>>: Clone + 'static {
    /// Trigger event ONLY if this filter passes!
    type Filter: StateFilter<State, Self>;
    type Action: ValidAction<PriorityMut<State>, Self::ActionInput> + Clone;
    type ActionInput: Clone;
    /// The action to execute when its event is triggered, along with its input.
    fn action(
        &self,
        state: &State,
        event: &Ev,
        value: <Self::Filter as StateFilter<State, Self>>::ValidOutput,
    ) -> (Self::Action, Self::ActionInput);
}

#[cfg(test)]
mod tests {
    use card_game_derive::StateFilterInput;

    use crate::{cards::ActionID, identifications::ActionIdentifier};

    use super::*;

    struct Game;
    struct Summoned;
    use crate as card_game;
    #[derive(StateFilterInput)]
    struct SummonedInput;
    impl Event<Game> for Summoned {
        type Input = SummonedInput;
    }
    #[derive(Clone)]
    struct SummonedListenerWhileInBattleZone;
    impl EventListener<Game, SummonedInput> for SummonedListenerWhileInBattleZone {
        type Filter = SummonedFilter;
        type Action = SummonedAction;
        fn action(self) -> Self::Action {
            SummonedAction
        }
    }
    #[derive(Clone)]
    struct EnteredBattleZoneListenerWhileInBattleZone;
    impl EventListener<Game, SummonedInput> for EnteredBattleZoneListenerWhileInBattleZone {
        type Filter = SummonedFilter;
        type Action = EnteredBattleZoneAction;
        fn action(self) -> Self::Action {
            EnteredBattleZoneAction
        }
    }
    #[derive(Clone)]
    enum SummonedEventListener {
        Summoned(SummonedListenerWhileInBattleZone),
        EnteredBattleZone(EnteredBattleZoneListenerWhileInBattleZone),
    }
    impl EventListener<Game, SummonedInput> for SummonedEventListener {
        type Filter = SummonedFilter;
        type Action = EnteredBattleZoneAction;
        fn action(self) -> Self::Action {
            EnteredBattleZoneAction
        }
    }
    pub enum AllSummonedActions {
        Summon(SummonedAction),
        Entered(EnteredBattleZoneAction),
    }
    impl ValidAction<Game, SummonedInput> for AllSummonedActions {
        type Filter = ();
        type Output = ();
        fn with_valid_input(
            self,
            state: Game,
            valid: <Self::Filter as StateFilter<Game, SummonedInput>>::ValidOutput,
        ) -> Self::Output {
        }
    }
    impl ActionIdentifier for AllSummonedActions {
        fn action_id() -> ActionID {
            match self {
                AllSummonedActions::Summon(summoned) => summoned.action_id(),
                AllSummonedActions::Entered(entered) => entered.action_id(),
            }
        }
    }
    enum AllSummonedActionsOutput {
        Summon(<SummonedAction as ValidAction<Game, SummonedInput>>::Output),
        Entered(<EnteredBattleZoneAction as ValidAction<Game, SummonedInput>>::Output),
    }
    struct SummonedFilter;
    impl StateFilter<Game, SummonedInput> for SummonedFilter {
        type ValidOutput = SummonedInput;
        //type Error = SummonedError;
        type Error = std::convert::Infallible;
        fn filter(state: &Game, value: SummonedInput) -> Result<Self::ValidOutput, Self::Error> {
            Ok(value)
        }
    }
    #[derive(thiserror::Error, Debug)]
    #[error("test summoned error")]
    struct SummonedError;
    struct SummonedAction;
    impl ValidAction<Game, SummonedInput> for SummonedAction {
        type Filter = SummonedFilter;
        type Output = ();
        fn with_valid_input(
            self,
            _state: Game,
            _valid: <Self::Filter as StateFilter<Game, SummonedInput>>::ValidOutput,
        ) -> Self::Output {
            ()
        }
        fn action_id() -> ActionID {
            ActionID::new("summoned_action")
        }
    }
    struct EnteredBattleZoneAction;
    impl ValidAction<Game, SummonedInput> for EnteredBattleZoneAction {
        type Filter = SummonedFilter;
        type Output = ();
        fn with_valid_input(
            self,
            _state: Game,
            _valid: <Self::Filter as StateFilter<Game, SummonedInput>>::ValidOutput,
        ) -> Self::Output {
            ()
        }
        fn action_id() -> ActionID {
            ActionID::new("test_summoned_action")
        }
    }
    struct GameSimultaneousEventHandler;
    impl SimultaneousEventHandler<Game, Summoned> for GameSimultaneousEventHandler {
        fn handle_simultaneous_events(self, event: TriggeredEvent<Game, Summoned>) {
            todo!()
        }
    }
    #[test]
    fn t() {
        let mut event_manager_0 =
            EventManager::<Game, Summoned, SummonedListenerWhileInBattleZone>::new();
        //let mut event_manager_1 = EventManager::new();
        //TriggeredEvent::<Game, Summoned, SummonedListenerWhileInBattleZone>::new(state, input);
    }
}
