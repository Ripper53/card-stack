use card_game::{
    cards::CardID,
    events::{
        AddEventListener, Event, EventAction, EventListener, GetEventManager, TriggeredEvent,
    },
    identifications::{PlayerID, SourceCardID},
    stack::priority::GetState,
};
use state_validation::{StateFilterInput, StateFilterInputConversion};

use crate::{
    Game, GameState,
    steps::{GetStateMut, MainStep},
};

#[derive(Clone, Copy)]
pub enum Summoned {
    Normal(NormalSummoned),
    Special(SpecialSummoned),
}
impl Summoned {
    pub fn player_id(&self) -> PlayerID {
        match self {
            Summoned::Normal(s) => s.player_id,
            Summoned::Special(s) => s.player_id,
        }
    }
    pub fn card_id(&self) -> CardID {
        match self {
            Summoned::Normal(s) => s.card_id,
            Summoned::Special(s) => s.card_id,
        }
    }
}
impl From<NormalSummoned> for Summoned {
    fn from(value: NormalSummoned) -> Self {
        Summoned::Normal(value)
    }
}
impl From<SpecialSummoned> for Summoned {
    fn from(value: SpecialSummoned) -> Self {
        Summoned::Special(value)
    }
}
impl<State: GetState<Game>> Event<State> for Summoned {
    type Input = Self;
    type Stackable = EventAction<State, Self, State>;
}
/*impl<State: GetState<Game>> GetEventManager<State, Summoned> for GameState<State> {
    type Output = Game;
    fn event_manager(&self) -> card_game::events::EventManager<State, Summoned, Self::Output> {
        self.0.state().event_manager().event_manager()
    }
}*/

#[derive(StateFilterInput, Clone, Copy)]
pub struct NormalSummoned {
    pub player_id: PlayerID,
    pub card_id: CardID,
}
impl From<Summoned> for NormalSummoned {
    fn from(value: Summoned) -> Self {
        NormalSummoned {
            player_id: value.player_id(),
            card_id: value.card_id(),
        }
    }
}
impl<State: GetState<Game>> Event<State> for NormalSummoned {
    type Input = Self;
    type Stackable = EventAction<State, Self, State>;
}

#[derive(StateFilterInput, Clone, Copy)]
pub struct SpecialSummoned {
    pub player_id: PlayerID,
    pub card_id: CardID,
}
impl From<Summoned> for SpecialSummoned {
    fn from(value: Summoned) -> Self {
        SpecialSummoned {
            player_id: value.player_id(),
            card_id: value.card_id(),
        }
    }
}
impl<State: GetState<Game> + GetEventManager<State, Self>> Event<State> for SpecialSummoned {
    type Input = Self;
    //type Stackable = SpecialSummonedStackable;
    //type Stackable = EventAction<State, Self, TriggeredEvent<State, Self>>;
    type Stackable = EventAction<State, Self, State::Output>;
}
/*pub enum SpecialSummonedStackable<State> {
    EventAction(EventAction<State, SpecialSummoned, TriggeredEvent<State, SpecialSummoned>>),
}
impl<State> From<EventAction<State, SpecialSummoned, TriggeredEvent<State, SpecialSummoned>>>
    for SpecialSummonedStackable<State>
{
    fn from(
        value: EventAction<State, SpecialSummoned, TriggeredEvent<State, SpecialSummoned>>,
    ) -> Self {
        SpecialSummonedStackable::EventAction(value)
    }
}*/
/*impl<State: GetState<Game>> GetEventManager<State, SpecialSummoned> for GameState<State> {
    type Output = Game;
    fn event_manager(
        &self,
    ) -> card_game::events::EventManager<State, SpecialSummoned, Self::Output> {
        self.0.state().event_manager().event_manager()
    }
}*/

/*struct A;
impl<State: GetState<Game>> EventListener<State, SpecialSummoned> for A {
    type Filter = ();
    type Action = ();
    fn action(self, event: &SpecialSummoned) -> Self::Action {}
}*/
