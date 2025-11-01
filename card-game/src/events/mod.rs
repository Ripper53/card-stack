use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::{create_valid_identification, identifications::SourceCardID};
use card_stack::{
    actions::{ActionSource, IncitingAction, IncitingActionInfo, StackAction},
    priority::{GetState, Priority, PriorityStack},
};
use state_validation::{StateFilter, StateFilterInput, ValidAction};

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

impl<EventState: 'static, Ev: Event<EventState>, Output: 'static>
    EventManager<EventState, Ev, Output>
{
    pub fn empty() -> Self {
        EventManager { events: Vec::new() }
    }
    pub fn count(&self) -> usize {
        self.events.len()
    }
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
        <Listener::Action as ValidAction<
            NewState,
            <Listener::Filter as StateFilter<NewState, NewEv::Input>>::ValidOutput,
        >>::Output: Into<Output> + 'static,
        EventState: GetState<NewState> + Into<NewState>,
        Ev: Into<NewEv>,
        Ev::Input: Into<NewEv::Input>,
        DynEventListener<
            NewState,
            NewEv,
            <Listener::Action as ValidAction<
                NewState,
                <Listener::Filter as StateFilter<NewState, NewEv::Input>>::ValidOutput,
            >>::Output,
        >: NewStateTrait<
                EventState,
                Ev,
                <Listener::Action as ValidAction<
                    NewState,
                    <Listener::Filter as StateFilter<NewState, NewEv::Input>>::ValidOutput,
                >>::Output,
            >,
        DynEventListener<
            EventState,
            Ev,
            <Listener::Action as ValidAction<
                NewState,
                <Listener::Filter as StateFilter<NewState, NewEv::Input>>::ValidOutput,
            >>::Output,
        >: NewOutputTrait<EventState, Ev, Output>,
    {
        let listener = DynEventListener::new(listener).new_state().new_output();
        self.events.push(listener);
    }
    pub(crate) fn collect_actions(
        &self,
        state: &EventState,
        event: &Ev,
        input: Ev::Input,
    ) -> CollectedActions<EventState, Ev, Output> {
        let actions = self
            .events
            .iter()
            .filter_map(|listener| {
                let filter = listener.filter.get_dyn_filter();
                if let Ok(action_input) = (filter)(&state, input.clone()) {
                    Some(listener.action(event, action_input))
                } else {
                    None
                }
            })
            .collect();
        CollectedActions {
            actions,
            _m: std::marker::PhantomData::default(),
        }
    }
}
struct CollectedActions<State, Ev, Output> {
    actions: Vec<DynAction<State, Output>>,
    _m: std::marker::PhantomData<Ev>,
}
impl<EventState, Ev: Event<EventState>, Output> CollectedActions<EventState, Ev, Output> {
    fn simultaneous_action_manager(
        self,
        state: EventState,
    ) -> SimultaneousActionManager<EventState, Ev, Output>
    where
        EventAction<EventState, Ev, Output>: Into<Ev::Stackable>,
    {
        SimultaneousActionManager {
            state,
            actions: self
                .actions
                .into_iter()
                .enumerate()
                .map(|(i, action)| (SimultaneousActionID(i), action))
                .collect(),
            _m: std::marker::PhantomData::default(),
        }
    }
}
pub struct SimultaneousActionManager<State, Ev: Event<State>, Output>
where
    EventAction<State, Ev, Output>: Into<Ev::Stackable>,
{
    state: State,
    actions: HashMap<SimultaneousActionID, DynAction<State, Output>>,
    _m: std::marker::PhantomData<Ev>,
}
pub struct SingleAction<State, Output> {
    state: State,
    action: DynAction<State, Output>,
}
impl<State, Output> SingleAction<State, Output> {
    pub fn resolve(self) -> EventActionResolution<State, Output> {
        match self.action.with_given_valid_input(self.state) {
            Ok(output) => EventActionResolution::Resolved(output),
            Err(e) => EventActionResolution::Fizzled {
                state: e.state,
                error: e.error,
            },
        }
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct SimultaneousActionID(usize);
use crate as card_game;
create_valid_identification!(ValidSimultaneousActionID, SimultaneousActionID);
impl<F> ValidSimultaneousActionID<F> {
    pub(crate) fn new(id: SimultaneousActionID) -> Self {
        ValidSimultaneousActionID(id, std::marker::PhantomData::default())
    }
}
impl ValidSimultaneousActionID<()> {
    pub fn try_new<State, Ev: Event<State>, Output>(
        simultaneous_action_manager: &SimultaneousActionManager<State, Ev, Output>,
        id: SimultaneousActionID,
    ) -> Option<Self>
    where
        EventAction<State, Ev, Output>: Into<Ev::Stackable>,
    {
        if simultaneous_action_manager.actions.contains_key(&id) {
            Some(ValidSimultaneousActionID::new(id))
        } else {
            None
        }
    }
}
impl<State, Ev: Event<State>, Output> SimultaneousActionManager<State, Ev, Output>
where
    EventAction<State, Ev, Output>: Into<Ev::Stackable>,
{
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
    /// Resolves in order.
    /// First element is put on the stack last.
    pub fn resolve(
        mut self,
        mut order: Vec<ValidSimultaneousActionID<()>>,
    ) -> Result<
        PriorityStack<State, EventAction<State, Ev, Output>>,
        ResolveSimultaneousActionsError<State, Ev, Output>,
    > {
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
        let mut stack = Priority::new(self.state).stack(EventAction::new(action));
        for action_id in order.iter().rev() {
            let action = self.actions.remove(&action_id.0).unwrap();
            stack = stack.stack(EventAction::new(action));
        }
        Ok(stack)
    }
}
pub struct EventAction<EventState, Ev: Event<EventState>, Output> {
    action: DynAction<EventState, Output>,
    _m: std::marker::PhantomData<Ev>,
}
impl<EventState, Ev: Event<EventState>, Output> EventAction<EventState, Ev, Output> {
    fn new(action: DynAction<EventState, Output>) -> Self {
        EventAction {
            action,
            _m: std::marker::PhantomData::default(),
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
{
    fn from(action: EventAction<EventState, Ev, Output>) -> Self {
        let filter = action.action.filter;
        let action_input = action.action.action_input;
        let action = action.action.action;
        EventAction::new(DynAction {
            action: Box::new(move |state, input| (action)(state.into(), input).into()),
            action_input,
            filter: Box::new(move |state, input| (filter)(state.state(), input)),
        })
    }
}
impl<EventState, Ev: Event<EventState>, Output> IncitingAction<EventState, Ev::Input>
    for EventAction<EventState, Ev, Output>
{
    type Requirement = ();
    fn resolve(
        self,
        priority: card_stack::priority::PriorityMut<Priority<EventState>>,
        _: <<Self::Requirement as card_stack::requirements::ActionRequirement<
            Priority<EventState>,
            Ev::Input,
        >>::Filter as StateFilter<Priority<EventState>, Ev::Input>>::ValidOutput,
    ) -> Self::Resolved {
        match self.action.with_given_valid_input(priority.take_state()) {
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
        match self.action.with_given_valid_input(state) {
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
pub enum ResolveSimultaneousActionsError<State, Ev: Event<State>, Output>
where
    EventAction<State, Ev, Output>: Into<Ev::Stackable>,
{
    #[error("not all actions are ordered")]
    NotAllActionsAreOrdered(SimultaneousActionManager<State, Ev, Output>),
}

pub struct TriggeredEvent<State, Ev: Event<State>> {
    state: State,
    event: Ev,
    input: Ev::Input,
}

impl<State: GetEventManager<State, Ev> + 'static, Ev: Event<State>> TriggeredEvent<State, Ev>
where
    EventAction<State, Ev, State::Output>: Into<Ev::Stackable>,
{
    pub fn new(state: State, event: Ev, input: Ev::Input) -> Self {
        TriggeredEvent {
            state,
            event,
            input,
        }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn event(&self) -> &Ev {
        &self.event
    }
    pub fn event_input(&self) -> &Ev::Input {
        &self.input
    }
    pub fn collect(self) -> TriggeredEventResolution<State, Ev> {
        let mut simultaneous_action_manager = self
            .state
            .event_manager()
            .collect_actions(&self.state, &self.event, self.input)
            .simultaneous_action_manager(self.state);
        match simultaneous_action_manager.simultaneous_action_count() {
            0 => TriggeredEventResolution::None(simultaneous_action_manager.state),
            1 => TriggeredEventResolution::Action(SingleAction {
                state: simultaneous_action_manager.state,
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
    /*pub fn consume<Listener: EventListener<State, Ev>>(self) -> EventConsume<State, Ev, Listener>
    where
        State: GetEventManager<State, Ev, Listener>,
        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput: StateFilterInput,
    {
        let event_manager = self.state.event_manager();
        match event_manager.events.len() {
            0 => EventConsume {
                kind: EventConsumeType::Finished(self.state),
                _m: std::marker::PhantomData::default(),
            },
            1 => {
                match <Listener::Filter as StateFilter<State, Ev::Input>>::filter(
                    &self.state,
                    self.input,
                ) {
                    Ok(valid_output) => {
                        match <Listener::Action as ValidAction<
                            State,
                            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
                        >>::Filter::filter(&self.state, valid_output)
                        {
                            Ok(valid_output) => {
                                let mut event_actions = event_manager
                                    .collect_actions(&self.state, &self.event, valid_output)
                                    .simultaneous_action_manager(self.state);
                                // UNWRAP: because length of events is 1, so there will be exactly 1 action.
                                let mut action = event_actions.actions.pop().unwrap();
                                let result = action.resolve().with_valid_input(
                                    event_actions.state,
                                    event_actions.valid_input,
                                );
                                EventConsume {
                                    kind: EventConsumeType::Result(result),
                                    _m: std::marker::PhantomData::default(),
                                }
                            }
                            Err(error) => EventConsume {
                                kind: EventConsumeType::ActionFizzle {
                                    state: self.state,
                                    error,
                                },
                                _m: std::marker::PhantomData::default(),
                            },
                        }
                    }
                    Err(error) => EventConsume {
                        kind: EventConsumeType::EventFizzle {
                            state: self.state,
                            error,
                        },
                        _m: std::marker::PhantomData::default(),
                    },
                }
            }
            _ => {
                match <Listener::Filter as StateFilter<State, Ev::Input>>::filter(
                    &self.state,
                    self.input,
                ) {
                    Ok(valid_output) => {
                        match <Listener::Action as ValidAction<
                            State,
                            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
                        >>::Filter::filter(&self.state, valid_output)
                        {
                            Ok(valid_output) => {
                                let event_actions = event_manager
                                    .collect_actions(&self.state, &self.event, valid_output)
                                    .simultaneous_action_manager(self.state);
                                EventConsume {
                                    kind: EventConsumeType::SimultaneousActions(event_actions),
                                    _m: std::marker::PhantomData::default(),
                                }
                            }
                            Err(error) => EventConsume {
                                kind: EventConsumeType::ActionFizzle {
                                    state: self.state,
                                    error,
                                },
                                _m: std::marker::PhantomData::default(),
                            },
                        }
                    }
                    Err(error) => EventConsume {
                        kind: EventConsumeType::EventFizzle {
                            state: self.state,
                            error,
                        },
                        _m: std::marker::PhantomData::default(),
                    },
                }
            }
        }
    }*/
}
pub enum TriggeredEventResolution<State: GetEventManager<State, Ev>, Ev: Event<State>>
where
    EventAction<State, Ev, State::Output>: Into<Ev::Stackable>,
{
    None(State),
    Action(SingleAction<State, State::Output>),
    SimultaneousActions(SimultaneousActionManager<State, Ev, State::Output>),
}
pub(crate) struct DynEventListener<State, Ev: Event<State>, Output> {
    get_dyn_action: Box<dyn GetDynActionTrait<State, Ev, Output>>,
    filter: Box<dyn GetDynStateFilterTrait<State, Ev::Input>>,
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
        {
            fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<NewState, NewEv, Output>> {
                Box::new(NewStateDynAction(self.0.dyn_clone()))
            }
            fn get_dyn_action(
                self: Box<Self>,
                event: NewEv,
                action_input: Box<dyn Any>,
            ) -> DynAction<NewState, Output> {
                let dyn_action = self.0.get_dyn_action(event.into(), action_input);
                let action = dyn_action.action;
                let filter = dyn_action.filter;
                DynAction {
                    action: Box::new(move |state, value| (action)(state.into(), value)),
                    action_input: dyn_action.action_input,
                    filter: Box::new(move |state, value| (filter)(state.state(), value)),
                }
            }
        }
        struct F<State, Input>(Box<dyn GetDynStateFilterTrait<State, Input>>);
        impl<
            State: 'static,
            NewState: GetState<State> + 'static,
            Input: 'static,
            NewInput: Into<Input> + 'static,
        > GetDynStateFilterTrait<NewState, NewInput> for F<State, Input>
        {
            fn dyn_clone(&self) -> Box<dyn GetDynStateFilterTrait<NewState, NewInput>> {
                Box::new(F(self.0.dyn_clone()))
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
            get_dyn_action: Box::new(NewStateDynAction(self.get_dyn_action)),
            filter: Box::new(F(self.filter)),
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
            get_dyn_action: Box::new(NewOutputAction::<State, Ev, Output>(
                self.get_dyn_action.dyn_clone(),
            )),
            filter: Box::new(DynStateFilter(self.filter)),
        }
    }
}
impl<State: 'static, Ev: Event<State>, Output: 'static> DynEventListener<State, Ev, Output> {
    pub(crate) fn new<Listener: EventListener<State, Ev>>(listener: Listener) -> Self
    where
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Output: Into<Output>,
    {
        DynEventListener {
            get_dyn_action: Box::new(GetDynAction { listener }),
            //get_dyn_action: todo!(),
            filter: Box::new(DynStaticStateFilter::new(
                |state: &State,
                 value: Ev::Input|
                 -> Result<Box<dyn Any>, Box<dyn std::error::Error>> {
                    match <Listener::Filter>::filter(state, value) {
                        Ok(result) => Ok(Box::new(result)),
                        Err(error) => Err(Box::new(error)),
                    }
                },
            )),
        }
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
        action_input: Box<dyn Any>,
    ) -> DynAction<State, Output>;
}
impl<State, Ev: Event<State>, Listener: EventListener<State, Ev>, Output>
    GetDynActionTrait<State, Ev, Output> for GetDynAction<Listener>
where
    <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput: 'static,
    <Listener::Filter as StateFilter<State, Ev::Input>>::Error: 'static,
    <<Listener::Action as ValidAction<
        State,
        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
    >>::Filter as StateFilter<
        State,
        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
    >>::ValidOutput: 'static,
    <<Listener::Action as ValidAction<
        State,
        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
    >>::Filter as StateFilter<
        State,
        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
    >>::Error: 'static,
    <Listener::Action as ValidAction<
        State,
        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
    >>::Output: Into<Output>,
{
    fn dyn_clone(&self) -> Box<dyn GetDynActionTrait<State, Ev, Output>> {
        Box::new(GetDynAction {
            listener: self.listener.clone(),
        })
    }
    fn get_dyn_action(
        self: Box<Self>,
        event: Ev,
        action_input: Box<dyn Any>,
    ) -> DynAction<State, Output> {
        DynAction {
            action: Box::new(move |state, valid| {
                self.listener
                    .action(&state, &event)
                    .with_valid_input(state, *valid.downcast().unwrap())
                    .into()
            }),
            action_input: Some(action_input),
            filter: Box::new(
                |state: &State,
                 value: Box<dyn Any>|
                 -> Result<Box<dyn Any>, Box<dyn std::error::Error>> {
                    match <<Listener::Action as ValidAction<
                        State,
                        <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
                    >>::Filter>::filter(state, *value.downcast().unwrap())
                    {
                        Ok(result) => Ok(Box::new(result)),
                        Err(error) => Err(Box::new(error)),
                    }
                },
            ),
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
    fn action(&self, event: &Ev, action_input: Box<dyn Any>) -> DynAction<EventState, Output> {
        self.get_dyn_action
            .dyn_clone()
            .get_dyn_action(event.clone(), action_input)
    }
}
struct DynAction<EventState, Output> {
    pub(crate) action: Box<dyn Fn(EventState, Box<dyn Any>) -> Output>,
    pub(crate) action_input: Option<Box<dyn Any>>,
    pub(crate) filter: Box<
        dyn for<'a> Fn(
            &'a EventState,
            Box<dyn Any>,
        ) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
    >,
}
struct ToBoxAnyFilter;
impl<State, Input: 'static> StateFilter<State, Input> for ToBoxAnyFilter {
    type ValidOutput = Box<dyn Any>;
    type Error = std::convert::Infallible;
    fn filter(_state: &State, value: Input) -> Result<Self::ValidOutput, Self::Error> {
        Ok(Box::new(value))
    }
}
impl<EventState, Input: 'static, Output> ValidAction<EventState, Input>
    for DynAction<EventState, Output>
{
    type Filter = ToBoxAnyFilter;
    type Output = Result<Output, DynStateError<EventState>>;
    fn with_valid_input(
        self,
        state: EventState,
        valid: <Self::Filter as StateFilter<EventState, Input>>::ValidOutput,
    ) -> Self::Output {
        match (self.filter)(&state, valid) {
            Ok(result) => Ok((self.action)(state, result)),
            Err(error) => Err(DynStateError { state, error }),
        }
    }
}
impl<EventState, Output> DynAction<EventState, Output> {
    pub(crate) fn with_given_valid_input(
        mut self,
        state: EventState,
    ) -> Result<Output, DynStateError<EventState>> {
        let action_input = self.action_input.take().unwrap();
        <DynAction<EventState, Output> as ValidAction<EventState, Box<dyn Any>>>::with_valid_input(
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
pub trait GetEventManager<State, Ev: Event<State>> {
    type Output;
    fn event_manager(&self) -> EventManager<State, Ev, Self::Output>;
}
pub trait AddEventListener<State, Ev: Event<State>> {
    type Output;
    fn add_listener<Listener: EventListener<State, Ev>>(&mut self, listener: Listener)
    where
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Output: Into<Self::Output>;
}

pub trait Event<State>: Clone + 'static {
    type Input: StateFilterInput + Clone;
    type Stackable;
    //fn event_id() -> EventID;
}

pub trait EventListenerConstructor<State, Ev: Event<State>>: EventListener<State, Ev> {
    type Input;
    fn new_listener(source_card_id: SourceCardID, input: Self::Input) -> Self;
}
pub trait EventListener<State, Ev: Event<State>>: Clone + 'static {
    /// Trigger event ONLY if this filter passes!
    type Filter: StateFilter<State, Ev::Input>;
    type Action: ValidAction<State, <Self::Filter as StateFilter<State, Ev::Input>>::ValidOutput>;
    /// The action to execute when its event is triggered.
    fn action(&self, state: &State, event: &Ev) -> Self::Action;
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
