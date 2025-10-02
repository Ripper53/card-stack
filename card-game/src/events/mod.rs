use std::{collections::HashMap, hash::Hash};

use crate::{
    cards::CardID,
    validation::{StateFilter, StateFilterInput, ValidAction},
};

pub struct EventManager<State, E: Event<State>, Listener: EventListener<State, E>> {
    events: Vec<Listener>,
    _m: std::marker::PhantomData<(State, E)>,
}

impl<State, E: Event<State>, Listener: EventListener<State, E>> EventManager<State, E, Listener> {
    pub fn new() -> Self {
        EventManager {
            events: Vec::new(),
            _m: std::marker::PhantomData::default(),
        }
    }
}
impl<State, E: Event<State>, Listener: EventListener<State, E>> Clone
    for EventManager<State, E, Listener>
{
    fn clone(&self) -> Self {
        EventManager {
            events: self.events.clone(),
            _m: std::marker::PhantomData::default(),
        }
    }
}

pub struct TriggeredEvent<State, E: Event<State>> {
    state: State,
    event: E,
    input: E::Input,
}

impl<State, E: Event<State>> TriggeredEvent<State, E> {
    pub fn new(state: State, event: E, input: E::Input) -> Self {
        TriggeredEvent {
            state,
            event,
            input,
        }
    }
    pub fn consume<Listener: EventListener<State, E>>(
        self,
        handler: impl SimultaneousEventHandler<State, E>,
    ) -> EventConsume<State, E, Listener>
    where
        State: GetEventManager<State, E, Listener>,
        <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput: StateFilterInput,
    {
        let event_manager = self.state.event_manager();
        match event_manager.events.len() {
            0 => EventConsume {
                kind: EventConsumeType::Finished(self.state),
                _m: std::marker::PhantomData::default(),
            },
            1 => {
                match <Listener::Filter as StateFilter<State, E::Input>>::filter(
                    &self.state,
                    self.input,
                ) {
                    Ok(valid_output) => {
                        match <Listener::Action as ValidAction<
                            State,
                            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
                        >>::Filter::filter(&self.state, valid_output)
                        {
                            Ok(valid_output) => {
                                let mut event_manager = event_manager.clone();
                                let listener = event_manager.events.pop().unwrap();
                                let r = listener
                                    .action(&self.event)
                                    .with_valid_input(self.state, valid_output);
                                EventConsume {
                                    kind: EventConsumeType::Result(r),
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
                //handler.handle_simultaneous_events();
                todo!()
            }
        }
    }
}
pub struct EventConsume<State, E: Event<State>, Listener: EventListener<State, E>> {
    kind: EventConsumeType<
        State,
        <Listener::Filter as StateFilter<State, E::Input>>::Error,
        <<Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
        >>::Filter as StateFilter<
            State,
            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
        >>::Error,
        <Listener::Action as ValidAction<
            State,
            <Listener::Filter as StateFilter<State, E::Input>>::ValidOutput,
        >>::Output,
    >,
    _m: std::marker::PhantomData<(E, Listener)>,
}
pub enum EventConsumeType<State, E0, E1, R> {
    Finished(State),
    EventFizzle { state: State, error: E0 },
    ActionFizzle { state: State, error: E1 },
    Result(R),
}
pub trait SimultaneousEventHandler<State, E: Event<State>> {
    fn handle_simultaneous_events(self, event: TriggeredEvent<State, E>);
}
pub trait GetEventManager<State, E: Event<State>, Listener: EventListener<State, E>> {
    fn event_manager(&self) -> &EventManager<State, E, Listener>;
}

impl<State, E: Event<State>, Listener: EventListener<State, E>> EventManager<State, E, Listener> {}

pub trait Event<State>: Hash {
    type Input: StateFilterInput;
    //fn event_id() -> EventID;
}

pub trait EventListener<State, E: Event<State>>: Clone
where
    <Self::Filter as StateFilter<State, E::Input>>::ValidOutput: StateFilterInput,
{
    type Action: ValidAction<State, <Self::Filter as StateFilter<State, E::Input>>::ValidOutput>;
    /// Trigger event ONLY if this filter passes!
    type Filter: StateFilter<State, E::Input>;
    /// The action to execute when its event is triggered.
    fn action(self, event: &E) -> Self::Action;
}

#[cfg(test)]
mod tests {
    use card_game_derive::StateFilterInput;

    use crate::cards::ActionID;

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
