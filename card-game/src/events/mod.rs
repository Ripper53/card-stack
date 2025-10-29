use std::{any::Any, hash::Hash};

use crate::{create_valid_identification, identifications::SourceCardID};
use card_stack::priority::GetState;
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
    pub fn add_listener<Listener: EventListener<EventState, Ev>>(&mut self, listener: Listener)
    where
        <Listener::Action as ValidAction<
            EventState,
            <Listener::Filter as StateFilter<EventState, Ev::Input>>::ValidOutput,
        >>::Output: Into<Output>,
    {
        self.events.push(DynEventListener::new(listener));
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
    ) -> SimultaneousActionManager<EventState, Ev, Output> {
        SimultaneousActionManager {
            state,
            actions: self
                .actions
                .into_iter()
                .map(|action| SimultaneousAction::Unresolved(action))
                .collect(),
            _m: std::marker::PhantomData::default(),
        }
    }
}
pub struct SimultaneousActionManager<State, E, Output> {
    state: State,
    actions: Vec<SimultaneousAction<State, Output>>,
    _m: std::marker::PhantomData<E>,
}
pub enum SimultaneousAction<State, Output> {
    Unresolved(DynAction<State, Output>),
    Resolved,
    Fizzled,
}
impl<State, Output> SimultaneousAction<State, Output> {
    /// PANIC: if action is not unresolved
    fn resolve(&mut self) -> DynAction<State, Output> {
        let action = std::mem::replace(self, SimultaneousAction::Resolved);
        if let SimultaneousAction::Unresolved(action) = action {
            action
        } else {
            unreachable!();
        }
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct SimultaneousActionID(usize);
use crate as card_game;
create_valid_identification!(ValidSimultaneousActionID, SimultaneousActionID);
pub struct Unresolved;
impl<F> ValidSimultaneousActionID<F> {
    pub(crate) fn new(id: SimultaneousActionID) -> Self {
        ValidSimultaneousActionID(id, std::marker::PhantomData::default())
    }
    pub fn try_new<State, E, Output>(
        simultaneous_action_manager: &SimultaneousActionManager<State, E, Output>,
        id: SimultaneousActionID,
    ) -> Option<Self> {
        if matches!(
            simultaneous_action_manager.actions.get(id.0),
            Some(SimultaneousAction::Unresolved(_))
        ) {
            Some(ValidSimultaneousActionID::new(id))
        } else {
            None
        }
    }
}
impl<State, Ev, Output> SimultaneousActionManager<State, Ev, Output> {
    pub fn simultaneous_action_ids(&self) -> impl Iterator<Item = ValidSimultaneousActionID<()>> {
        self.actions
            .iter()
            .enumerate()
            .map(|(index, _action)| ValidSimultaneousActionID::new(SimultaneousActionID(index)))
    }
    pub fn unresolved_simultaneous_action_ids(
        &self,
    ) -> impl Iterator<Item = ValidSimultaneousActionID<Unresolved>> {
        self.actions
            .iter()
            .enumerate()
            .filter_map(|(index, action)| {
                if matches!(action, SimultaneousAction::Unresolved(_)) {
                    Some(ValidSimultaneousActionID::new(SimultaneousActionID(index)))
                } else {
                    None
                }
            })
    }
    pub fn resolve(
        mut self,
        action_id: ValidSimultaneousActionID<Unresolved>,
    ) -> Result<Output, DynStateError<State>> {
        let action = self.actions.get_mut(action_id.0.0).unwrap().resolve();
        action.with_given_valid_input(self.state)
    }
}

pub struct TriggeredEvent<State, E: Event<State>> {
    state: State,
    event: E,
    input: E::Input,
}

impl<State: GetEventManager<State, Ev> + 'static, Ev: Event<State>> TriggeredEvent<State, Ev> {
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
    pub fn read(self) {
        let collected_actions = self
            .state
            .event_manager()
            .collect_actions(&self.state, &self.event, self.input)
            .simultaneous_action_manager(self.state);
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
    action: Box<dyn Fn(EventState, Box<dyn Any>) -> Output>,
    action_input: Option<Box<dyn Any>>,
    filter: Box<
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
pub struct EventConsumeBuilder<State, Ev: Event<State>, Output> {
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
}
pub trait GetEventManager<State, Ev: Event<State>> {
    type Output;
    fn event_manager(&self) -> EventManager<State, Ev, Self::Output>;
}
pub trait GetEventManagerMut<State, Ev: Event<State>> {
    type Output;
    fn event_manager_mut(&mut self) -> &mut EventManager<State, Ev, Self::Output>;
}

pub trait Event<State>: Clone + 'static {
    type Input: StateFilterInput + Clone;
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
