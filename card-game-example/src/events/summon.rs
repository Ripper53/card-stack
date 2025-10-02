use card_game::{
    cards::CardID,
    events::{Event, EventListener},
    identifications::PlayerID,
    stack::priority::GetState,
};

use crate::{Game, filters::FilterInput};

#[derive(Hash)]
pub struct SpecialSummoned;

impl<State: GetState<Game>> Event<State> for SpecialSummoned {
    type Input = FilterInput<(PlayerID, CardID)>;
}

/*struct A;
impl<State: GetState<Game>> EventListener<State, SpecialSummoned> for A {
    type Filter = ();
    type Action = ();
    fn action(self, event: &SpecialSummoned) -> Self::Action {}
}*/
