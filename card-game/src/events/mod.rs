use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::{create_valid_identification, identifications::SourceCardID};
use card_stack::{
    actions::{ActionSource, IncitingAction, IncitingActionInfo, StackAction},
    priority::{CombineState, GetState, Priority, PriorityStack, TakeState},
};
use state_validation::{StateFilter, ValidAction};

pub struct EventManager<State: 'static, Ev: Event<State>, Output> {
    events: Vec<DynEventListener<State, Ev, Output>>,
}
impl<State: 'static, Ev: Event<State>, Output> Clone for EventManager<State, Ev, Output> {
    fn clone(&self) -> Self {
        EventManager {
            events: self.events.clone(),
        }
    }
}
impl<State: 'static, Ev: Event<State>, Output> Default for EventManager<State, Ev, Output> {
    fn default() -> Self {
        Self::empty()
    }
}
impl<State: 'static, Ev: Event<State>, Output> EventManager<State, Ev, Output> {
    pub fn empty() -> Self {
        EventManager { events: Vec::new() }
    }
    pub fn count(&self) -> usize {
        self.events.len()
    }
}
impl<EventState: 'static, Ev: Event<EventState>, Output: 'static>
    EventManager<EventState, Ev, Output>
{
    pub fn new_combined<
        StateA,
        StateB,
        EvA: Event<StateA>,
        EvB: Event<StateB>,
        OutputA: Into<Output> + 'static,
        OutputB: Into<Output> + 'static,
    >(
        event_manager_a: &EventManager<StateA, EvA, OutputA>,
        event_manager_b: &EventManager<StateB, EvB, OutputB>,
    ) -> EventManager<EventState, Ev, Output>
    where
        EventState: Into<StateA> + Into<StateB>,
        Ev: Into<EvA> + Into<EvB>,
        Ev::Input: Into<EvA::Input> + Into<EvB::Input>,
        DynEventListener<StateA, EvA, OutputA>: NewStateTrait<EventState, Ev, OutputA>,
        DynEventListener<StateB, EvB, OutputB>: NewStateTrait<EventState, Ev, OutputB>,
        DynEventListener<EventState, Ev, OutputA>: NewOutputTrait<EventState, Ev, Output>,
        DynEventListener<EventState, Ev, OutputB>: NewOutputTrait<EventState, Ev, Output>,
    {
        let mut events: Vec<DynEventListener<EventState, Ev, Output>> =
            Vec::with_capacity(event_manager_a.events.len() + event_manager_b.events.len());
        for event in event_manager_a.events.iter().cloned() {
            events.push(event.new_state().new_output());
        }
        for event in event_manager_b.events.iter().cloned() {
            events.push(event.new_state().new_output());
        }
        EventManager { events }
    }
    pub fn combine<
        NewState: 'static,
        NewEv: Event<NewState> + Event<EventState>,
        NewOutput: Into<Output> + 'static,
    >(
        mut self,
        new_event_manager: &EventManager<NewState, NewEv, NewOutput>,
    ) -> Self
    where
        EventState: Into<NewState>,
        Ev: Into<NewEv>,
        Ev::Input: Into<<NewEv as Event<EventState>>::Input>,
        DynEventListener<NewState, NewEv, NewOutput>: NewStateTrait<EventState, Ev, NewOutput>,
        DynEventListener<EventState, Ev, NewOutput>: NewOutputTrait<EventState, Ev, Output>,
    {
        for event in new_event_manager.events.iter().cloned() {
            self.events.push(event.new_state().new_output());
        }
        self
    }
    pub(crate) fn new(events: Vec<DynEventListener<EventState, Ev, Output>>) -> Self {
        EventManager { events }
    }
    pub fn add_listener<
        NewState: 'static,
        NewEv: Event<NewState>,
        Listener: EventListener<NewState, NewEv>,
    >(
        &mut self,
        listener: Listener,
    ) where
        <Listener::Action as ValidAction<NewState, NewEv::Input>>::Output: Into<Output> + 'static,
        EventState: GetState<NewState> + Into<NewState>,
        Ev: Into<NewEv>,
        Ev::Input: Into<NewEv::Input>,
        NewEv::Input: Into<Ev::Input>,
        DynEventListener<
            NewState,
            NewEv,
            <Listener::Action as ValidAction<NewState, NewEv::Input>>::Output,
        >: NewStateTrait<
                EventState,
                Ev,
                <Listener::Action as ValidAction<NewState, NewEv::Input>>::Output,
            >,
        DynEventListener<
            EventState,
            Ev,
            <Listener::Action as ValidAction<NewState, NewEv::Input>>::Output,
        >: NewOutputTrait<EventState, Ev, Output>,
    {
        let listener = DynEventListener::new(listener).new_state().new_output();
        self.events.push(listener);
    }
    pub(crate) fn collect_actions(
        &self,
        state: &EventState,
        event: &Ev,
        event_input: Ev::Input,
    ) -> CollectedActions<EventState, Ev, Output> {
        let actions = self
            .events
            .iter()
            .filter_map(|listener| {
                let filter = listener.filter.get_dyn_filter();
                if let Ok(initial_filter_output) = (filter)(state, listener.clone_listener()) {
                    let action = listener.action(event, initial_filter_output);
                    if (action.action_filter)(state, event_input.clone()).is_ok() {
                        return Some(action);
                    }
                }
                None
            })
            .collect();
        CollectedActions {
            event_input,
            actions,
        }
    }
}
struct CollectedActions<State, Ev: Event<State>, Output> {
    event_input: Ev::Input,
    actions: Vec<DynAction<State, Ev, Output>>,
}
impl<EventState, Ev: Event<EventState>, Output> CollectedActions<EventState, Ev, Output> {
    fn simultaneous_action_manager<State>(
        self,
        state: State,
    ) -> SimultaneousActionManager<State, Ev, Output>
    where
        State: GetEventManager<Ev, State = EventState>,
        Ev: Event<State, Input = <Ev as Event<EventState>>::Input>,
        EventAction<EventState, Ev, Output>: Into<<Ev as Event<EventState>>::Stackable>,
    {
        SimultaneousActionManager {
            state,
            event_input: self.event_input,
            actions: self
                .actions
                .into_iter()
                .enumerate()
                .map(|(i, action)| (SimultaneousActionID(i), action))
                .collect(),
        }
    }
}
pub struct SimultaneousActionManager<
    State: GetEventManager<Ev>,
    Ev: Event<State> + Event<State::State>,
    Output,
> where
    EventAction<State::State, Ev, Output>: Into<<Ev as Event<State::State>>::Stackable>,
{
    state: State,
    event_input: <Ev as Event<State::State>>::Input,
    actions: HashMap<SimultaneousActionID, DynAction<State::State, Ev, Output>>,
}
pub struct SingleAction<State: GetEventManager<Ev>, Ev: Event<State> + Event<State::State>, Output>
{
    state: State,
    event_input: <Ev as Event<State::State>>::Input,
    action: DynAction<State::State, Ev, Output>,
}
impl<State: GetEventManager<Ev>, Ev: Event<State> + Event<State::State>, Output>
    SingleAction<State, Ev, Output>
where
    <Ev as Event<State>>::Input: 'static,
    <Ev as Event<State::State>>::Input: 'static,
{
    pub fn resolve(
        self,
    ) -> EventActionResolution<
        State,
        <Output as CombineState<
            <State as TakeState<<State as GetEventManager<Ev>>::State>>::Remainder,
        >>::Combined,
    >
    where
        State: TakeState<<State as GetEventManager<Ev>>::State>,
        Output:
            CombineState<<State as TakeState<<State as GetEventManager<Ev>>::State>>::Remainder>,
        <State as TakeState<<State as GetEventManager<Ev>>::State>>::Remainder:
            CombineState<<State as GetEventManager<Ev>>::State, Combined = State>,
    {
        let (state, remainder) = self.state.take_state();
        match self.action.with_given_valid_input(state, self.event_input) {
            Ok(output) => EventActionResolution::Resolved(output.combine(remainder)),
            Err(e) => EventActionResolution::Fizzled {
                state: remainder.combine(e.state),
                error: e.error,
            },
        }
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
impl<State: GetEventManager<Ev>, Ev: Event<State> + Event<State::State>, Output>
    SimultaneousActionManager<State, Ev, Output>
where
    EventAction<State::State, Ev, Output>: Into<<Ev as Event<State::State>>::Stackable>,
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
impl<State: GetEventManager<Ev>, Ev: Event<State> + Event<State::State>, Output>
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
        PriorityStack<State, EventAction<State::State, Ev, Output>>,
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
    {
        let (state, remainder) = self.state.take_state();
        let action = self.actions.remove(&action_id.0).unwrap();
        if let Ok(output) = action.with_given_valid_input(state, self.event_input) {
            output.combine(remainder)
        } else {
            unreachable!()
        }
    }
}
pub struct EventAction<EventState, Ev: Event<EventState>, Output> {
    event_input: Ev::Input,
    action: DynAction<EventState, Ev, Output>,
}
impl<EventState, Ev: Event<EventState>, Output> std::fmt::Debug
    for EventAction<EventState, Ev, Output>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventAction {{ ... }}")
    }
}
impl<EventState, Ev: Event<EventState>, Output> EventAction<EventState, Ev, Output> {
    fn new(event_input: Ev::Input, action: DynAction<EventState, Ev, Output>) -> Self {
        EventAction {
            event_input,
            action,
        }
    }
}
pub trait FromEventAction<EventState, Ev: Event<EventState>, Output> {
    fn from(action: EventAction<EventState, Ev, Output>) -> Self;
}
impl<
    EventState: 'static,
    NewState: GetState<EventState> + Into<EventState>,
    Ev: Event<EventState> + Event<NewState>,
    Output: Into<NewOutput> + 'static,
    NewOutput,
> FromEventAction<EventState, Ev, Output> for EventAction<NewState, Ev, NewOutput>
where
    <Ev as Event<EventState>>::Input: Into<<Ev as Event<NewState>>::Input>,
    <Ev as Event<NewState>>::Input: Into<<Ev as Event<EventState>>::Input>,
{
    fn from(action: EventAction<EventState, Ev, Output>) -> Self {
        let event_input = action.event_input;
        let initial_filter_output = action.action.initial_filter_output;
        let action_filter = action.action.action_filter;
        let action = action.action.action;
        EventAction::new(
            event_input.into(),
            DynAction {
                initial_filter_output,
                action_filter: Box::new(move |state, input| {
                    (action_filter)(state.state(), input.into())
                }),
                action: Box::new(move |state, value, valid| {
                    (action)(state.into(), value, valid).into()
                }),
            },
        )
    }
}
impl<EventState, Ev: Event<EventState>, Output> IncitingAction<EventState, ()>
    for EventAction<EventState, Ev, Output>
where
    Ev::Input: 'static,
{
    type Requirement = ();
    fn resolve(
        self,
        priority: card_stack::priority::PriorityMut<Priority<EventState>>,
        _: <<Self::Requirement as card_stack::requirements::ActionRequirement<
            Priority<EventState>,
            (),
        >>::Filter as StateFilter<Priority<EventState>, ()>>::ValidOutput,
    ) -> Self::Resolved {
        match self
            .action
            .with_given_valid_input(priority.take_state(), self.event_input)
        {
            Ok(output) => EventActionResolution::Resolved(output),
            Err(e) => EventActionResolution::Fizzled {
                state: e.state,
                error: e.error,
            },
        }
    }
}
impl<EventState, Ev: Event<EventState>, Output> IncitingActionInfo<EventState>
    for EventAction<EventState, Ev, Output>
{
    type Resolved = EventActionResolution<EventState, Output>;
    type Stackable = Ev::Stackable;
}
impl<
    T: IncitingAction<EventState, Ev::Input>
        + IncitingActionInfo<Output, Stackable = <T as IncitingActionInfo<EventState>>::Stackable>,
    EventState,
    Ev: Event<EventState>,
    Output,
> StackAction<EventState, (), T> for EventAction<EventState, Ev, Output>
where
    Ev::Input: 'static,
{
    type Requirement = ();
    type Resolved = EventActionResolution<PriorityStack<EventState, T>, PriorityStack<Output, T>>;
    fn resolve(
        self,
        priority: card_stack::priority::PriorityMut<
            card_stack::priority::PriorityStack<EventState, T>,
        >,
        (): <<Self::Requirement as card_stack::requirements::ActionRequirement<
                    card_stack::priority::PriorityStack<EventState, T>,
                    (),
                >>::Filter as StateFilter<card_stack::priority::PriorityStack<EventState, T>, ()>>::ValidOutput,
    ) -> Self::Resolved {
        let (state, stack) = priority.take_priority().take_contents();
        match self.action.with_given_valid_input(state, self.event_input) {
            Ok(output) => EventActionResolution::Resolved(PriorityStack::from_stack(
                Priority::new(output),
                stack.into_state(),
            )),
            Err(e) => EventActionResolution::Fizzled {
                state: PriorityStack::from_stack(Priority::new(e.state), stack),
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
pub enum ResolveSimultaneousActionsError<
    State: GetEventManager<Ev>,
    Ev: Event<State> + Event<State::State>,
    Output,
> where
    EventAction<State, Ev, Output>: Into<<Ev as Event<State>>::Stackable>,
    EventAction<State::State, Ev, Output>: Into<<Ev as Event<State::State>>::Stackable>,
{
    #[error("not all actions are ordered")]
    NotAllActionsAreOrdered(SimultaneousActionManager<State, Ev, Output>),
}

pub struct TriggeredEvent<State, Ev: Event<State>> {
    state: State,
    event: Ev,
    input: Ev::Input,
}

impl<State, Ev: Event<State>> TriggeredEvent<State, Ev> {
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn event(&self) -> &Ev {
        &self.event
    }
    pub fn event_input(&self) -> &<Ev as Event<State>>::Input {
        &self.input
    }
}
impl<
    State: GetState<State::State> + GetEventManager<Ev> + 'static,
    Ev: Event<State>
        + Event<
            State::State,
            Input = <Ev as Event<State>>::Input,
            Stackable = <Ev as Event<State>>::Stackable,
        >,
> TriggeredEvent<State, Ev>
where
    EventAction<<State as GetEventManager<Ev>>::State, Ev, State::Output>:
        Into<<Ev as Event<<State as GetEventManager<Ev>>::State>>::Stackable>,
{
    pub fn new(state: State, event: Ev, input: <Ev as Event<State>>::Input) -> Self {
        TriggeredEvent {
            state,
            event,
            input,
        }
    }
    pub fn collect(self) -> TriggeredEventResolution<State, Ev> {
        let mut simultaneous_action_manager = self
            .state
            .event_manager()
            .collect_actions(self.state.state(), &self.event, self.input)
            .simultaneous_action_manager(self.state);
        match simultaneous_action_manager.simultaneous_action_count() {
            0 => TriggeredEventResolution::None(simultaneous_action_manager.state),
            1 => TriggeredEventResolution::Action(SingleAction {
                state: simultaneous_action_manager.state,
                event_input: simultaneous_action_manager.event_input,
                action: simultaneous_action_manager
                    .actions
                    .into_iter()
                    .next()
                    .unwrap()
                    .1,
            }),
            _ => TriggeredEventResolution::SimultaneousActions(simultaneous_action_manager),
        }
    }
}
pub enum TriggeredEventResolution<
    State: GetEventManager<Ev>,
    Ev: Event<State> + Event<State::State>,
> where
    EventAction<State::State, Ev, State::Output>: Into<<Ev as Event<State::State>>::Stackable>,
{
    None(State),
    Action(SingleAction<State, Ev, State::Output>),
    SimultaneousActions(SimultaneousActionManager<State, Ev, State::Output>),
}
pub(crate) struct DynEventListener<State, Ev: Event<State>, Output> {
    listener: ListenerClone,
    get_dyn_action: Box<dyn GetDynActionTrait<State, Ev, Output>>,
    filter: Box<dyn GetDynStateFilterTrait<State, Box<dyn Any>>>,
    //filter: for<'a> fn(&'a State, Ev::Input) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
}
trait NewStateTrait<NewState, NewEv: Event<NewState>, Output> {
    fn new_state(self) -> DynEventListener<NewState, NewEv, Output>;
}
impl<
    State: 'static,
    NewState: GetState<State> + Into<State> + 'static,
    Ev: Event<State>,
    NewEv: Event<NewState> + Into<Ev>,
    Output: 'static,
> NewStateTrait<NewState, NewEv, Output> for DynEventListener<State, Ev, Output>
where
    Ev::Input: Into<NewEv::Input>,
    NewEv::Input: Into<Ev::Input>,
{
    fn new_state(self) -> DynEventListener<NewState, NewEv, Output> {
        struct NewStateDynAction<State, Ev, Output>(Box<dyn GetDynActionTrait<State, Ev, Output>>);
        impl<
            State: 'static,
            NewState: GetState<State> + Into<State> + 'static,
            Ev: Event<State>,
            NewEv: Event<NewState> + Into<Ev>,
            Output: 'static,
        > GetDynActionTrait<NewState, NewEv, Output> for NewStateDynAction<State, Ev, Output>
        where
            Ev::Input: Into<NewEv::Input>,
            NewEv::Input: Into<Ev::Input>,
        {
            fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<NewState, NewEv, Output>> {
                Box::new(NewStateDynAction(self.0.dyn_clone()))
            }
            fn get_dyn_action(
                self: Box<Self>,
                event: NewEv,
                initial_filter_output: Box<dyn Any>,
            ) -> DynAction<NewState, NewEv, Output> {
                let dyn_action = self.0.get_dyn_action(event.into(), initial_filter_output);
                let initial_filter_output = dyn_action.initial_filter_output;
                let action_filter = dyn_action.action_filter;
                let action = dyn_action.action;
                DynAction {
                    initial_filter_output,
                    action_filter: Box::new(move |state, value| {
                        (action_filter)(state.state(), value.into())
                    }),
                    action: Box::new(move |state, value, valid| {
                        (action)(state.into(), value, valid)
                    }),
                }
            }
        }
        struct DynFilter<State, Input>(Box<dyn GetDynStateFilterTrait<State, Input>>);
        impl<
            State: 'static,
            NewState: GetState<State> + 'static,
            Input: 'static,
            NewInput: Into<Input> + 'static,
        > GetDynStateFilterTrait<NewState, NewInput> for DynFilter<State, Input>
        {
            fn dyn_clone(&self) -> Box<dyn GetDynStateFilterTrait<NewState, NewInput>> {
                Box::new(DynFilter(self.0.dyn_clone()))
            }
            fn get_dyn_filter<'a>(
                &'a self,
            ) -> Box<
                dyn for<'b> Fn(
                        &'b NewState,
                        NewInput,
                    )
                        -> Result<Box<dyn Any>, Box<dyn std::error::Error>>
                    + 'a,
            > {
                Box::new(|state, value| (self.0.get_dyn_filter())(state.state(), value.into()))
            }
        }
        DynEventListener {
            listener: self.listener,
            get_dyn_action: Box::new(NewStateDynAction(self.get_dyn_action)),
            filter: Box::new(DynFilter(self.filter)),
        }
    }
}
trait NewOutputTrait<State, Ev: Event<State>, NewOutput> {
    fn new_output(self) -> DynEventListener<State, Ev, NewOutput>;
}
impl<State: 'static, Ev: Event<State>, Output: 'static, NewOutput: 'static>
    NewOutputTrait<State, Ev, NewOutput> for DynEventListener<State, Ev, Output>
where
    Output: Into<NewOutput>,
{
    fn new_output(self) -> DynEventListener<State, Ev, NewOutput> {
        struct NewOutputAction<State, Ev, Output>(Box<dyn GetDynActionTrait<State, Ev, Output>>);
        impl<
            State: 'static,
            NewEv: Event<State> + Into<Ev>,
            Ev: Event<State>,
            NewOutput,
            Output: Into<NewOutput> + 'static,
        > GetDynActionTrait<State, NewEv, NewOutput> for NewOutputAction<State, Ev, Output>
        where
            Ev::Input: Into<NewEv::Input>,
            NewEv::Input: Into<Ev::Input>,
        {
            fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<State, NewEv, NewOutput>> {
                Box::new(NewOutputAction(self.0.dyn_clone()))
            }
            fn get_dyn_action(
                self: Box<Self>,
                event: NewEv,
                initial_filter_output: Box<dyn Any>,
            ) -> DynAction<State, NewEv, NewOutput> {
                let inner_action = self.0.get_dyn_action(event.into(), initial_filter_output);
                let initial_filter_output = inner_action.initial_filter_output;
                let old_action = inner_action.action;
                let action_filter = inner_action.action_filter;
                DynAction {
                    initial_filter_output,
                    action_filter: Box::new(move |state, value| {
                        (action_filter)(state, value.into())
                    }),
                    action: Box::new(move |state, value, valid| {
                        (old_action)(state, value, valid).into()
                    }),
                }
            }
        }
        struct DynStateFilter<State, Input>(Box<dyn GetDynStateFilterTrait<State, Input>>);
        impl<State: 'static, Input: 'static, NewInput: Into<Input> + 'static>
            GetDynStateFilterTrait<State, NewInput> for DynStateFilter<State, Input>
        {
            fn dyn_clone(&self) -> Box<dyn GetDynStateFilterTrait<State, NewInput>> {
                Box::new(DynStateFilter(self.0.dyn_clone()))
            }
            fn get_dyn_filter<'a>(
                &'a self,
            ) -> Box<
                dyn for<'b> Fn(
                        &'b State,
                        NewInput,
                    )
                        -> Result<Box<dyn Any>, Box<dyn std::error::Error>>
                    + 'a,
            > {
                Box::new(|state, input| (self.0.get_dyn_filter())(state, input.into()))
            }
        }
        DynEventListener {
            listener: self.listener,
            get_dyn_action: Box::new(NewOutputAction::<State, Ev, Output>(
                self.get_dyn_action.dyn_clone(),
            )),
            filter: Box::new(DynStateFilter(self.filter)),
        }
    }
}
trait DynAnyClone: Any {
    fn dyn_any_clone(&self) -> Box<dyn DynAnyClone>;
}
impl<T: Clone + 'static> DynAnyClone for T {
    fn dyn_any_clone(&self) -> Box<dyn DynAnyClone> {
        Box::new(self.clone())
    }
}
trait DynListenerClone {
    fn clone_listener(&self) -> Box<dyn Any>;
}
struct ListenerClone(Box<dyn DynAnyClone>);
impl Clone for ListenerClone {
    fn clone(&self) -> Self {
        ListenerClone((*self.0).dyn_any_clone())
    }
}
impl DynListenerClone for ListenerClone {
    fn clone_listener(&self) -> Box<dyn Any> {
        self.0.dyn_any_clone()
    }
}
impl<State: 'static, Ev: Event<State>, Output: 'static> DynEventListener<State, Ev, Output> {
    pub(crate) fn new<Listener: EventListener<State, Ev>>(listener: Listener) -> Self
    where
        <Listener::Action as ValidAction<State, Ev::Input>>::Output: Into<Output>,
    {
        DynEventListener {
            listener: ListenerClone(Box::new(listener.clone())),
            get_dyn_action: Box::new(GetDynAction { listener }),
            filter: Box::new(DynStaticStateFilter::new(
                move |state: &State,
                      listener: Box<dyn Any>|
                      -> Result<Box<dyn Any>, Box<dyn std::error::Error>> {
                    match <Listener::Filter>::filter(state, *listener.downcast().unwrap()) {
                        Ok(result) => Ok(Box::new(result)),
                        Err(error) => Err(Box::new(error)),
                    }
                },
            )),
        }
    }
    pub(crate) fn clone_listener(&self) -> Box<dyn Any> {
        self.listener.clone_listener()
    }
    /*pub(crate) fn new_output<NewEv: Event<State> + Into<Ev>, NewOutput>(
        &self,
    ) -> DynEventListener<State, NewEv, NewOutput>
    where
        NewEv::Input: Into<Ev::Input>,
        Output: Into<NewOutput>,
        Box<dyn GetDynStateFilterTrait<State, Ev::Input>>:
            DynCastInto<State, Ev::Input, NewEv::Input>,
    {
        struct NewOutputAction<State, Ev, Output>(Box<dyn GetDynActionTrait<State, Ev, Output>>);
        impl<
            State: 'static,
            NewEv: Event<State> + Into<Ev>,
            Ev: Event<State>,
            NewOutput,
            Output: Into<NewOutput> + 'static,
        > GetDynActionTrait<State, NewEv, NewOutput> for NewOutputAction<State, Ev, Output>
        where
            NewEv::Input: Into<Ev::Input>,
        {
            fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<State, NewEv, NewOutput>> {
                Box::new(NewOutputAction(self.0.dyn_clone()))
            }
            fn get_dyn_action(
                self: Box<Self>,
                event: NewEv,
                action_input: Box<dyn Any>,
            ) -> DynAction<State, NewOutput> {
                let inner_action = self.0.get_dyn_action(event.into(), action_input);
                let old_action = inner_action.action;
                DynAction {
                    action: Box::new(move |state, valid| (old_action)(state, valid).into()),
                    action_input: inner_action.action_input,
                    filter: inner_action.filter,
                    /*filter: |_state: &State,
                             value: Box<dyn Any>|
                     -> Result<Box<dyn Any>, Box<dyn std::error::Error>> {
                         Ok(value)
                    },*/
                }
            }
        }
        DynEventListener {
            get_dyn_action: Box::new(NewOutputAction::<State, Ev, Output>(
                self.get_dyn_action.dyn_clone(),
            )),
            filter: self.filter.dyn_into(),
            //filter: GetDynStateFilterTrait::<State, NewEv::Input>::dyn_clone(&self.filter),
        }
    }*/
}
impl<State: 'static, Ev: Event<State>, Output> Clone for DynEventListener<State, Ev, Output> {
    fn clone(&self) -> Self {
        DynEventListener {
            listener: self.listener.clone(),
            get_dyn_action: self.get_dyn_action.dyn_clone(),
            filter: self.filter.dyn_clone(),
        }
    }
}
#[derive(Clone)]
struct GetDynAction<Listener: Clone> {
    listener: Listener,
}
trait GetDynActionTrait<State, Ev: Event<State>, Output> {
    fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<State, Ev, Output>>;
    fn get_dyn_action(
        self: Box<Self>,
        event: Ev,
        initial_filter_output: Box<dyn Any>,
    ) -> DynAction<State, Ev, Output>;
}
impl<State, Ev: Event<State>, Listener: EventListener<State, Ev>, Output>
    GetDynActionTrait<State, Ev, Output> for GetDynAction<Listener>
where
    <Listener::Filter as StateFilter<State, Listener>>::ValidOutput: 'static,
    <Listener::Filter as StateFilter<State, Listener>>::Error: 'static,
    <<Listener::Action as ValidAction<State, Ev::Input>>::Filter as StateFilter<
        State,
        Ev::Input,
    >>::ValidOutput: 'static,
    <<Listener::Action as ValidAction<State, Ev::Input>>::Filter as StateFilter<
        State,
        Ev::Input,
    >>::Error: 'static,
    <Listener::Action as ValidAction<State, Ev::Input>>::Output: Into<Output>,
{
    fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<State, Ev, Output>> {
        Box::new(GetDynAction {
            listener: self.listener.clone(),
        })
    }
    fn get_dyn_action(
        self: Box<Self>,
        event: Ev,
        initial_filter_output: Box<dyn Any>,
    ) -> DynAction<State, Ev, Output> {
        DynAction {
            initial_filter_output: Some(initial_filter_output),
            action_filter: Box::new(
                |state: &State,
                 value: Ev::Input|
                 -> Result<Box<dyn Any>, Box<dyn std::error::Error>> {
                    match <<Listener::Action as ValidAction<State, Ev::Input>>::Filter>::filter(
                        state, value,
                    ) {
                        Ok(result) => Ok(Box::new(result)),
                        Err(error) => Err(Box::new(error)),
                    }
                },
            ),
            action: Box::new(move |state, value, valid| {
                self.listener
                    .action(&state, &event, *value.downcast().unwrap())
                    .with_valid_input(state, *valid.downcast().unwrap())
                    .into()
            }),
        }
    }
}
/*impl<State: 'static, Ev: Event<State>, Output: 'static> EventListener<State, Ev>
    for DynEventListener<State, Ev, Output>
where
    Ev::Input: 'static,
{
    type Filter = ();
    type Action = DynAction<State, Output>;
    fn action(&self, _state: &State, event: &Ev) -> Self::Action {
        self.get_dyn_action
            .dyn_clone()
            .get_dyn_action(event.clone())
    }
}*/
impl<EventState, Ev: Event<EventState>, Output> DynEventListener<EventState, Ev, Output> {
    fn action(
        &self,
        event: &Ev,
        initial_filter_output: Box<dyn Any>,
    ) -> DynAction<EventState, Ev, Output> {
        self.get_dyn_action
            .dyn_clone()
            .get_dyn_action(event.clone(), initial_filter_output)
    }
}
struct DynAction<EventState, Ev: Event<EventState>, Output> {
    pub(crate) initial_filter_output: Option<Box<dyn Any>>,
    pub(crate) action_filter: Box<
        dyn for<'a> Fn(
            &'a EventState,
            Ev::Input,
        ) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
    >,
    pub(crate) action: Box<dyn Fn(EventState, Box<dyn Any>, Box<dyn Any>) -> Output>,
}
struct ToBoxAnyFilter;
impl<State, Input: 'static> StateFilter<State, Input> for ToBoxAnyFilter {
    type ValidOutput = Box<dyn Any>;
    type Error = std::convert::Infallible;
    fn filter(_state: &State, value: Input) -> Result<Self::ValidOutput, Self::Error> {
        Ok(Box::new(value))
    }
}
impl<EventState, Ev: Event<EventState>, Output> ValidAction<EventState, Ev::Input>
    for DynAction<EventState, Ev, Output>
where
    Ev::Input: 'static,
{
    type Filter = ();
    type Output = Result<Output, DynStateError<EventState>>;
    fn with_valid_input(
        mut self,
        state: EventState,
        action_input: <Self::Filter as StateFilter<EventState, Ev::Input>>::ValidOutput,
    ) -> Self::Output {
        match (self.action_filter)(&state, action_input) {
            Ok(result) => Ok((self.action)(
                state,
                self.initial_filter_output.take().unwrap(),
                result,
            )),
            Err(error) => Err(DynStateError { state, error }),
        }
    }
}
impl<EventState, Ev: Event<EventState>, Output> DynAction<EventState, Ev, Output>
where
    Ev::Input: 'static,
{
    pub(crate) fn with_given_valid_input(
        mut self,
        state: EventState,
        action_input: Ev::Input,
    ) -> Result<Output, DynStateError<EventState>> {
        <DynAction<EventState, Ev, Output> as ValidAction<EventState, Ev::Input>>::with_valid_input(
            self,
            state,
            action_input,
        )
    }
}
struct DynStateError<State> {
    state: State,
    error: Box<dyn std::error::Error>,
}
struct DynStaticStateFilter<State, Input> {
    filter: for<'a> fn(&'a State, Input) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
}
impl<State, Input> DynStaticStateFilter<State, Input> {
    fn new(
        filter: for<'a> fn(&'a State, Input) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
    ) -> Self {
        DynStaticStateFilter { filter }
    }
}
struct DynChainStateFilter<State, Input> {
    filter: for<'a> fn(&'a State, Input) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
}
trait GetDynStateFilterTrait<State, Input> {
    fn dyn_clone(&self) -> Box<dyn GetDynStateFilterTrait<State, Input>>;
    fn get_dyn_filter<'a>(
        &'a self,
    ) -> Box<
        dyn for<'b> Fn(&'b State, Input) -> Result<Box<dyn Any>, Box<dyn std::error::Error>> + 'a,
    >;
}
impl<State: 'static, Input: 'static, NewInput: Into<Input> + 'static>
    GetDynStateFilterTrait<State, NewInput> for DynStaticStateFilter<State, Input>
{
    fn dyn_clone(&self) -> Box<dyn GetDynStateFilterTrait<State, NewInput>> {
        Box::new(DynStaticStateFilter {
            filter: self.filter,
        })
    }
    fn get_dyn_filter<'a>(
        &'a self,
    ) -> Box<
        dyn for<'b> Fn(&'b State, NewInput) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>
            + 'a,
    > {
        Box::new(|state, input| (self.filter)(state, input.into()))
    }
}
/*trait DynCastInto<State, OldInput, NewInput: Into<OldInput>> {
    fn dyn_into(self: Box<Self>) -> Box<dyn GetDynStateFilterTrait<State, NewInput>>;
}
impl<State, Input, T: GetDynStateFilterTrait<State, Input> + ?Sized, NewInput: Into<Input>>
    DynCastInto<State, Input, NewInput> for T
{
    fn dyn_into(self: Box<Self>) -> Box<dyn GetDynStateFilterTrait<State, NewInput>> {
        struct NewStateFilter<State, Input>(Box<dyn GetDynStateFilterTrait<State, Input>>);
        impl<State: 'static, Input, NewInput: Into<Input>> GetDynStateFilterTrait<State, NewInput>
            for NewStateFilter<State, Input>
        {
            fn dyn_clone(&self) -> Box<dyn GetDynStateFilterTrait<State, NewInput>> {
                Box::new(NewStateFilter(self.0.dyn_clone()))
            }
            fn get_dyn_filter<'a>(
                &'a self,
            ) -> Box<
                dyn for<'b> Fn(
                        &'b State,
                        NewInput,
                    )
                        -> Result<Box<dyn Any>, Box<dyn std::error::Error>>
                    + 'a,
            > {
                let filter = self.0.get_dyn_filter();
                Box::new(move |state, input| (filter)(state, input.into()))
            }
        }
        Box::new(NewStateFilter::<State, NewInput>(self))
    }
}*/
/*pub struct EventConsumeBuilder<State, Ev: Event<State>, Output> {
    listeners: Vec<DynEventListener<State, Ev, Output>>,
}
pub struct EventConsume<State, Ev: Event<State>, Listener: EventListener<State, Ev>> {
    kind: EventConsumeType<
        State,
        <Listener::Filter as StateFilter<State, Ev::Input>>::Error,
        <<Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Filter as StateFilter<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Error,
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Output,
        SimultaneousActionManager<State, Ev, Listener>,
    >,
    _m: std::marker::PhantomData<(Ev, Listener)>,
}
pub enum EventConsumeType<State, E0, E1, R, ActionManager> {
    Finished(State),
    EventFizzle { state: State, error: E0 },
    ActionFizzle { state: State, error: E1 },
    Result(R),
    SimultaneousActions(ActionManager),
}*/
pub trait GetEventManager<Ev: Event<Self::State>> {
    type State;
    type Output;
    fn event_manager(&self) -> EventManager<Self::State, Ev, Self::Output>;
}
impl<
    State: GetState<<State as GetEventManager<Ev>>::State> + GetEventManager<Ev>,
    Ev: Event<<State as GetEventManager<Ev>>::State>,
    IncitingAction: IncitingActionInfo<State>,
> GetEventManager<Ev> for PriorityStack<State, IncitingAction>
where
    <State as GetEventManager<Ev>>::State:
        GetEventManager<Ev, State = <State as GetEventManager<Ev>>::State>,
{
    type State = <State as GetEventManager<Ev>>::State;
    type Output = <<State as GetEventManager<Ev>>::State as GetEventManager<Ev>>::Output;
    fn event_manager(&self) -> EventManager<Self::State, Ev, Self::Output> {
        self.state().event_manager()
    }
}

pub trait AddEventListener<State, Ev: Event<State>> {
    type Output;
    fn add_listener<Listener: EventListener<State, Ev>>(&mut self, listener: Listener)
    where
        <Listener::Action as ValidAction<State, Ev::Input>>::Output: Into<Self::Output>;
}

pub trait Event<State>: Clone + 'static {
    type Input: Clone;
    type Stackable;
    //fn event_id() -> EventID;
}

pub trait EventListenerConstructor<State, Ev: Event<State>>: EventListener<State, Ev> {
    type Input;
    fn new_listener(source_card_id: SourceCardID, input: Self::Input) -> Self;
}
pub trait EventListener<State, Ev: Event<State>>: Clone + 'static {
    /// Trigger event ONLY if this filter passes!
    type Filter: StateFilter<State, Self>;
    type Action: ValidAction<State, Ev::Input>;
    /// The action to execute when its event is triggered.
    fn action(
        &self,
        state: &State,
        event: &Ev,
        value: <Self::Filter as StateFilter<State, Self>>::ValidOutput,
    ) -> Self::Action;
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
