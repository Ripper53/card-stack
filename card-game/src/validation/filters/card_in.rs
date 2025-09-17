use crate::{
    cards::CardID,
    identifications::{PlayerID, ValidPlayerID},
    validation::StateFilter,
    zones::{ValidCardID, Zone},
};

// TODO: maybe don't implement any filters in core crate?
/*pub struct CardIn<Z: Zone>(std::marker::PhantomData<Z>);
impl<State, Z: Zone + 'static> StateFilter for CardIn<Z> {
    type State = State;
    type Input<'a> = (PlayerID, CardID);
    type ValidOutput<'a> = (ValidPlayerID<'a>, ValidCardID<'a, Self>);
    fn filter<'a>(
        state: &Self::State,
        (player_id, card_id): Self::ValidOutput<'a>,
    ) -> Option<<Self::ValidOutput<'a> as crate::validation::ValidFor<Self>>::Valid<'a>> {
        Z::filter(state, value)
    }
}*/
/*macro_rules! impl_valid_state_for_card {
    ($($t: ident,)*) => {
        impl<'a, State, Z: Zone + for<'b> StateFilter<State = State, Value<'b> = (PlayerID, CardID)> + 'static $(,$t: for<'b> StateFilter<State = State, Value<'b> = (PlayerID, CardID)> + 'static)*> crate::validation::ValidState<'a, State, (CardIn<Z> $(,$t)*)> {
            pub fn execute<Action: crate::validation::ValidAction<State = State, Filter = (CardIn<Z> $(,$t)*)>>(
                mut self,
                valid_action: Action,
            ) -> Action::Output {
                valid_action.with_valid_input(
                    self.state,
                    (
                        ValidPlayerID::new(self.value.0),
                        ValidCardID::new(self.value.1),
                    ),
                )
            }
            pub fn valid_player_id(&self) -> ValidPlayerID<'_> {
                ValidPlayerID::new(self.value.0)
            }
            pub fn valid_card_id(&self) -> ValidCardID<'_, (Z $(,$t)*)> {
                ValidCardID::new(self.value.1)
            }
        }
    };
}
impl_valid_state_for_card!();
impl_valid_state_for_card!(T0,);
impl_valid_state_for_card!(T0, T1,);
impl_valid_state_for_card!(T0, T1, T2,);
impl_valid_state_for_card!(T0, T1, T2, T3,);*/
