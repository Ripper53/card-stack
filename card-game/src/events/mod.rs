use std::{any::Any, hash::Hash};

use crate::{create_valid_identification, identifications::SourceCardID};
use state_validation::{StateFilter, StateFilterInput, ValidAction};

#[derive(Clone)]
pub struct EventManager<State: 'static, Ev: Event<State>, Output> {
    events: Vec<DynEventListener<State, Ev, Output>>,
}
pub struct SimultaneousActionManager<State, E: Event<State>, Output> {
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
impl<F> ValidSimultaneousActionID<F> {
    pub(crate) fn new(id: SimultaneousActionID) -> Self {
        ValidSimultaneousActionID(id, std::marker::PhantomData::default())
    }
}

impl<State: 'static, Ev: Event<State>, Output: 'static> EventManager<State, Ev, Output> {
    pub fn empty() -> Self {
        EventManager { events: Vec::new() }
    }
    pub fn new_combined<
        EvA: Event<State>,
        EvB: Event<State>,
        OutputA: Into<Output> + 'static,
        OutputB: Into<Output> + 'static,
    >(
        event_manager_a: &EventManager<State, EvA, OutputA>,
        event_manager_b: &EventManager<State, EvB, OutputB>,
    ) -> EventManager<State, Ev, Output>
    where
        Ev: Into<EvA> + Into<EvB>,
        Ev::Input: Into<EvA::Input> + Into<EvB::Input>,
    {
        let mut events: Vec<DynEventListener<State, Ev, Output>> =
            Vec::with_capacity(event_manager_a.events.len() + event_manager_b.events.len());
        for event in event_manager_a.events.iter() {
            events.push(event.new_output::<Ev, Output>())
        }
        for event in event_manager_b.events.iter() {
            events.push(event.new_output::<Ev, Output>())
        }
        EventManager { events }
    }
    pub fn combine<NewEv: Event<State>, NewOutput: Into<Output> + 'static>(
        mut self,
        new_event_manager: &EventManager<State, Ev, Output>,
    ) -> Self {
        for event in new_event_manager.events.iter() {
            self.events.push(event.new_output())
        }
        self
    }
    pub(crate) fn new(events: Vec<DynEventListener<State, Ev, Output>>) -> Self {
        EventManager { events }
    }
    pub fn add_listener<Listener: EventListener<State, Ev>>(&mut self, listener: Listener)
    where
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
        >>::Output: Into<Output>,
    {
        self.events.push(DynEventListener::new(listener));
    }
    pub(crate) fn collect_actions(
        &self,
        state: &State,
        event: &Ev,
        input: Ev::Input,
    ) -> CollectedActions<State, Ev, Output> {
        let actions = self
            .events
            .iter()
            .filter_map(|listener| {
                let filter = listener.filter.get_dyn_filter();
                if let Ok(action_input) = (filter)(state, input.clone()) {
                    Some(listener.action(state, event, action_input))
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
struct CollectedActions<State, Ev: Event<State>, Output> {
    actions: Vec<DynAction<State, Output>>,
    _m: std::marker::PhantomData<Ev>,
}
impl<State, Ev: Event<State>, Output> CollectedActions<State, Ev, Output> {
    pub fn simultaneous_action_manager(
        self,
        state: State,
    ) -> SimultaneousActionManager<State, Ev, Output> {
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
impl<State, Ev: Event<State>, Output> SimultaneousActionManager<State, Ev, Output> {
    pub fn simultaneous_action_ids(&self) -> impl Iterator<Item = ValidSimultaneousActionID<()>> {
        self.actions
            .iter()
            .enumerate()
            .map(|(index, _action)| ValidSimultaneousActionID::new(SimultaneousActionID(index)))
    }
    pub fn resolve<F>(
        mut self,
        action_id: ValidSimultaneousActionID<F>,
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

impl<State, Ev: Event<State>> TriggeredEvent<State, Ev> {
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
    pub(crate) fn new_output<NewEv: Event<State> + Into<Ev>, NewOutput>(
        &self,
    ) -> DynEventListener<State, NewEv, NewOutput>
    where
        NewEv::Input: Into<Ev::Input>,
        Output: Into<NewOutput>,
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
            filter: self.filter.dyn_clone(),
            //filter: GetDynStateFilterTrait::<State, NewEv::Input>::dyn_clone(&self.filter),
        }
    }
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
            filter: |state: &State,
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
impl<State, Ev: Event<State>, Output> DynEventListener<State, Ev, Output> {
    fn action(
        &self,
        _state: &State,
        event: &Ev,
        action_input: Box<dyn Any>,
    ) -> DynAction<State, Output> {
        self.get_dyn_action
            .dyn_clone()
            .get_dyn_action(event.clone(), action_input)
    }
}
struct DynAction<State, Output> {
    action: Box<dyn Fn(State, Box<dyn Any>) -> Output>,
    action_input: Option<Box<dyn Any>>,
    filter: for<'a> fn(&'a State, Box<dyn Any>) -> Result<Box<dyn Any>, Box<dyn std::error::Error>>,
}
struct ToBoxAnyFilter;
impl<State, Input: 'static> StateFilter<State, Input> for ToBoxAnyFilter {
    type ValidOutput = Box<dyn Any>;
    type Error = std::convert::Infallible;
    fn filter(_state: &State, value: Input) -> Result<Self::ValidOutput, Self::Error> {
        Ok(Box::new(value))
    }
}
impl<State, Input: 'static, Output> ValidAction<State, Input> for DynAction<State, Output> {
    type Filter = ToBoxAnyFilter;
    type Output = Result<Output, DynStateError<State>>;
    fn with_valid_input(
        self,
        state: State,
        valid: <Self::Filter as StateFilter<State, Input>>::ValidOutput,
    ) -> Self::Output {
        match (self.filter)(&state, valid) {
            Ok(result) => Ok((self.action)(state, result)),
            Err(error) => Err(DynStateError { state, error }),
        }
    }
}
impl<State, Output> DynAction<State, Output> {
    pub(crate) fn with_given_valid_input(
        mut self,
        state: State,
    ) -> Result<Output, DynStateError<State>> {
        let action_input = self.action_input.take().unwrap();
        <DynAction<State, Output> as ValidAction<State, Box<dyn Any>>>::with_valid_input(
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
pub trait SimultaneousEventHandler<State, E: Event<State>> {
    fn handle_simultaneous_events(self, event: TriggeredEvent<State, E>);
}
pub trait GetEventManager<State, Ev: Event<State>> {
    type Output;
    fn event_manager(&self) -> EventManager<State, Ev, Self::Output>;
}
pub trait GetEventManagerMut<State, Ev: Event<State>>: GetEventManager<State, Ev> {
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
