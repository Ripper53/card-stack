use card_game::{
    cards::CardID,
    events::{Event, EventListener, GetEventManagerMut},
    identifications::{PlayerID, SourceCardID},
    stack::priority::GetState,
};

use crate::{Game, filters::FilterInput, steps::GetStateMut};

#[derive(Clone)]
pub struct SpecialSummoned(pub CardID);

impl<State: GetState<Game>> Event<State> for SpecialSummoned {
    type Input = FilterInput<(PlayerID, CardID)>;
}

/*struct A;
impl<State: GetState<Game>> EventListener<State, SpecialSummoned> for A {
    type Filter = ();
    type Action = ();
    fn action(self, event: &SpecialSummoned) -> Self::Action {}
}*/
