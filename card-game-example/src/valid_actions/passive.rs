use card_game::{
    cards::CardID,
    events::{Event, EventListener, EventListenerConstructor},
    identifications::SourceCardID,
};
use state_validation::{Condition, StateFilter, ValidAction};

use crate::{Game, steps::GetStateMut};

/*
#[derive(Clone)]
pub struct Passive<T: Clone> {
    source_card_id: CardID,
    action: T,
}

impl<T: Clone> Passive<T> {
    pub fn new(source_card_id: CardID, action: T) -> Self {
        Passive {
            source_card_id,
            action,
        }
    }
}
pub trait PassiveAction<State, Input> {
    type Filter: StateFilter<State, Input>;
}
impl<
    State: GetStateMut<Game>,
    T: Clone
        + PassiveAction<State, Ev::Input>
        + ValidAction<State, <<T as PassiveAction<State, Ev::Input>>::Filter as StateFilter<State, Ev::Input>>::ValidOutput>
        + 'static,
    Ev: Event<State>,
> EventListener<State, Ev> for Passive<T>
{
    type Filter = (
        Condition<Ev::Input, <T as PassiveAction<State, Ev::Input>>::Filter>,
        Condition<
            <<T as PassiveAction<State, Ev::Input>>::Filter as StateFilter<State, Ev::Input>>::ValidOutput,
            <T as ValidAction<State, <<T as PassiveAction<State, Ev::Input>>::Filter as StateFilter<State, Ev::Input>>::ValidOutput>>::Filter,
        >,
    );
    type Action = T;
    fn action(&self, _state: &State, _event: &Ev) -> Self::Action {
        self.action.clone()
    }
}
impl<State: GetStateMut<Game>, T: Clone
        + PassiveAction<State, Ev::Input>
        + ValidAction<State, <<T as PassiveAction<State, Ev::Input>>::Filter as StateFilter<State, Ev::Input>>::ValidOutput>
        + 'static,
     Ev: Event<State>,
> EventListenerConstructor<State, Ev>
    for Passive<T>
{
    type Input = T;
    fn new_listener(source_card_id: SourceCardID, action: Self::Input) -> Self {
        Passive {
            source_card_id: source_card_id.0,
            action,
        }
    }
}*/
