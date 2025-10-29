use card_game::{
    StateFilterInput,
    cards::CardID,
    events::{Event, EventListener, GetEventManager, GetEventManagerMut},
    identifications::{PlayerID, SourceCardID},
    stack::priority::GetState,
    validation::StateFilterInputConversion,
};

use crate::{
    Game, GameState,
    filters::FilterInput,
    steps::{GetStateMut, MainStep},
};

#[derive(StateFilterInput, Clone, Copy)]
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
}
/*impl<State: GetState<Game>> GetEventManager<State, Summoned> for GameState<State> {
    type Output = Game;
    fn event_manager(&self) -> card_game::events::EventManager<State, Summoned, Self::Output> {
        self.0.state().event_manager().event_manager()
    }
}*/
impl StateFilterInputConversion<PlayerID> for Summoned {
    type Remainder = FilterInput<CardID>;
    fn split_take(self) -> (PlayerID, Self::Remainder) {
        (self.player_id(), FilterInput(self.card_id()))
    }
}
impl StateFilterInputConversion<CardID> for Summoned {
    type Remainder = FilterInput<PlayerID>;
    fn split_take(self) -> (CardID, Self::Remainder) {
        (self.card_id(), FilterInput(self.player_id()))
    }
}
impl StateFilterInputConversion<(PlayerID, CardID)> for Summoned {
    type Remainder = FilterInput<()>;
    fn split_take(self) -> ((PlayerID, CardID), Self::Remainder) {
        ((self.player_id(), self.card_id()), FilterInput(()))
    }
}

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
impl<State: GetState<Game>> Event<State> for SpecialSummoned {
    type Input = Self;
}
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
