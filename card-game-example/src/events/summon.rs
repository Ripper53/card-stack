use card_game::{
    StateFilterInput,
    cards::CardID,
    events::{Event, EventListener, GetEventManagerMut},
    identifications::{PlayerID, SourceCardID},
    stack::priority::GetState,
    validation::StateFilterInputConversion,
};

use crate::{Game, filters::FilterInput, steps::GetStateMut};

#[derive(StateFilterInput, Clone, Copy)]
pub struct Summoned {
    pub player_id: PlayerID,
    pub card_id: CardID,
}
impl<State: GetState<Game>> Event<State> for Summoned {
    type Input = Self;
}
impl StateFilterInputConversion<PlayerID> for Summoned {
    type Remainder = FilterInput<CardID>;
    fn split_take(self) -> (PlayerID, Self::Remainder) {
        (self.player_id, FilterInput(self.card_id))
    }
}
impl StateFilterInputConversion<CardID> for Summoned {
    type Remainder = FilterInput<PlayerID>;
    fn split_take(self) -> (CardID, Self::Remainder) {
        (self.card_id, FilterInput(self.player_id))
    }
}
impl StateFilterInputConversion<(PlayerID, CardID)> for Summoned {
    type Remainder = FilterInput<()>;
    fn split_take(self) -> ((PlayerID, CardID), Self::Remainder) {
        ((self.player_id, self.card_id), FilterInput(()))
    }
}

#[derive(Clone, Copy)]
pub struct NormalSummoned {
    pub player_id: PlayerID,
    pub card_id: CardID,
}
impl From<Summoned> for NormalSummoned {
    fn from(value: Summoned) -> Self {
        NormalSummoned {
            player_id: value.player_id,
            card_id: value.card_id,
        }
    }
}
impl<State: GetState<Game>> Event<State> for NormalSummoned {
    type Input = Summoned;
}

#[derive(Clone, Copy)]
pub struct SpecialSummoned {
    pub player_id: PlayerID,
    pub card_id: CardID,
}
impl From<Summoned> for SpecialSummoned {
    fn from(value: Summoned) -> Self {
        SpecialSummoned {
            player_id: value.player_id,
            card_id: value.card_id,
        }
    }
}
impl<State: GetState<Game>> Event<State> for SpecialSummoned {
    type Input = Summoned;
}

/*struct A;
impl<State: GetState<Game>> EventListener<State, SpecialSummoned> for A {
    type Filter = ();
    type Action = ();
    fn action(self, event: &SpecialSummoned) -> Self::Action {}
}*/
