use std::collections::HashMap;

use crate::{
    cards::CardID,
    validation::{StateFilter, StateFilterInput, ValidAction},
};

pub struct EventManager<State, E: Event<State>, Listener: EventListener<State, E::Input>> {
    events: Vec<Listener>,
    _m: std::marker::PhantomData<(State, E)>,
}

impl<State, E: Event<State>, Listener: EventListener<State, E::Input>>
    EventManager<State, E, Listener>
{
    pub fn new() -> Self {
        EventManager {
            events: Vec::new(),
            _m: std::marker::PhantomData::default(),
        }
    }
}
impl<State, E: Event<State>, Listener: EventListener<State, E::Input>> Clone
    for EventManager<State, E, Listener>
{
    fn clone(&self) -> Self {
        EventManager {
            events: self.events.clone(),
            _m: std::marker::PhantomData::default(),
        }
    }
}

pub struct TriggeredEvent<
    State,
    E: Event<State>,
    Listener: EventListener<State, E::Input>,
    Handler: SimultaneousEventHandler,
> {
    state: State,
    input: E::Input,
    _m: std::marker::PhantomData<(E, Listener, Handler)>,
}

impl<
    State: GetEventManager<State, E, Listener>,
    E: Event<State>,
    Listener: EventListener<State, E::Input>,
    Handler: SimultaneousEventHandler,
> TriggeredEvent<State, E, Listener, Handler>
{
    pub fn new(state: State, input: E::Input) -> Self {
        TriggeredEvent {
            state,
            input,
            _m: std::marker::PhantomData::default(),
        }
    }
    pub fn consume(self) -> State {
        let valid_output =
            <Listener::Filter as StateFilter<State, E::Input>>::filter(&self.state, self.input)
                .unwrap();
        let event_manager = self.state.event_manager();
        match event_manager.events.len() {
            0 => self.state,
            1 => {
                let valid_output = <Listener::Action as ValidAction<
                    State,
                    <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
                >>::Filter::filter(&self.state, valid_output)
                .unwrap();
                let mut e = event_manager.clone();
                e.events
                    .pop()
                    .unwrap()
                    .action()
                    .with_valid_input(self.state, valid_output);
                todo!()
            }
            _ => {
                Handler::handle_simultaneous_events();
                todo!()
            }
        }
    }
}
pub trait SimultaneousEventHandler {
    fn handle_simultaneous_events();
}
pub trait GetEventManager<State, E: Event<State>, Listener: EventListener<State, E::Input>> {
    fn event_manager(&self) -> &EventManager<State, E, Listener>;
}

impl<State, E: Event<State>, Listener: EventListener<State, E::Input>>
    EventManager<State, E, Listener>
{
}

pub trait Event<State> {
    type Input: StateFilterInput;
    //fn event_id() -> EventID;
}

pub trait EventListener<State, Input: StateFilterInput>: Clone
where
    <Self::Filter as StateFilter<State, Input>>::ValidOutput: StateFilterInput,
{
    type Action: ValidAction<State, <Self::Filter as StateFilter<State, Input>>::ValidOutput>;
    /// Trigger event ONLY if this filter passes!
    type Filter: StateFilter<State, Input>;
    /// The action to execute when its event is triggered.
    fn action(self) -> Self::Action;
}

#[cfg(test)]
mod tests {
    use card_game_derive::StateFilterInput;

    use crate::cards::ActionID;

    use super::*;

    struct Summoned;
    use crate as card_game;
    #[derive(StateFilterInput)]
    struct SummonedInput;
    impl Event<()> for Summoned {
        type Input = SummonedInput;
    }
    #[derive(Clone)]
    struct SummonedListener;
    impl EventListener<(), SummonedInput> for SummonedListener {
        type Action = SummonedAction;
        type Filter = SummonedFilter;
        fn action(self) -> Self::Action {
            SummonedAction
        }
    }
    struct SummonedFilter;
    impl StateFilter<(), SummonedInput> for SummonedFilter {
        type ValidOutput = SummonedInput;
        //type Error = SummonedError;
        type Error = std::convert::Infallible;
        fn filter(state: &(), value: SummonedInput) -> Result<Self::ValidOutput, Self::Error> {
            Ok(value)
        }
    }
    #[derive(thiserror::Error, Debug)]
    #[error("test summoned error")]
    struct SummonedError;
    struct SummonedAction;
    impl ValidAction<(), SummonedInput> for SummonedAction {
        type Filter = SummonedFilter;
        type Output = ();
        fn with_valid_input(
            self,
            _state: (),
            _valid: <Self::Filter as StateFilter<(), SummonedInput>>::ValidOutput,
        ) -> Self::Output {
            ()
        }
        fn action_id() -> ActionID {
            ActionID::new("test_summoned_action")
        }
    }
    #[test]
    fn t() {
        let mut event_manager_0 = EventManager::<(), Summoned, SummonedListener>::new();
        //let mut event_manager_1 = EventManager::new();
        //TriggeredEvent::new(state, input)
    }
}
