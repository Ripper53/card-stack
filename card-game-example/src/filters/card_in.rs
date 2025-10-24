use card_game::{
    cards::CardID,
    identifications::{
        ActivePlayer, CardDoesNotExist, GetValidCardIDFromZone, PlayerDoesNotExist, PlayerID,
        ValidCardID, ValidPlayerID,
    },
    stack::priority::GetState,
    steps::Step,
    validation::{StateFilter, StateFilterInput, StateFilterTwoChainError},
    zones::Zone,
};

use crate::{
    Game,
    filters::{FilterInput, For},
    steps::MainStep,
    valid_actions::{Tribute, ValidTribute},
    zones::{GetZone, monster::MonsterZone},
};

pub struct CardIn<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, Z: GetZone> StateFilter<State, FilterInput<(PlayerID, CardID)>>
    for CardIn<Z>
{
    type ValidOutput = FilterInput<(ValidPlayerID<()>, ValidCardID<Self>)>;
    type Error = PlayerOrCardError;
    fn filter(
        state: &State,
        FilterInput((player_id, card_id)): FilterInput<(PlayerID, CardID)>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        let state = state.state();
        let valid_player_id = ValidPlayerID::try_new(&state.player_manager, player_id)?;
        let valid_card_id = ValidCardID::try_new(card_id, Z::get_zone(state, &valid_player_id))?;
        Ok(FilterInput((valid_player_id, valid_card_id)))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum PlayerOrCardError {
    #[error(transparent)]
    Player(#[from] PlayerDoesNotExist),
    #[error(transparent)]
    Card(#[from] CardDoesNotExist),
}

impl<State: GetState<Game>, Z: GetZone, F> StateFilter<State, (ValidPlayerID<F>, CardID)>
    for CardIn<Z>
{
    type ValidOutput = (ValidPlayerID<F>, ValidCardID<Self>);
    type Error = CardDoesNotExist;
    fn filter(
        state: &State,
        (valid_player_id, card_id): (ValidPlayerID<F>, CardID),
    ) -> Result<Self::ValidOutput, Self::Error> {
        let valid_card_id =
            ValidCardID::try_new(card_id, Z::get_zone(state.state(), &valid_player_id))?;
        Ok((valid_player_id, valid_card_id))
    }
}
impl<State: GetState<Game>, Z: GetZone, F>
    StateFilter<State, FilterInput<(ValidPlayerID<F>, CardID)>> for CardIn<Z>
{
    type ValidOutput = FilterInput<(ValidPlayerID<F>, ValidCardID<Self>)>;
    type Error = CardDoesNotExist;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, card_id)): FilterInput<(ValidPlayerID<F>, CardID)>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        let valid_card_id =
            ValidCardID::try_new(card_id, Z::get_zone(state.state(), &valid_player_id))?;
        Ok(FilterInput((valid_player_id, valid_card_id)))
    }
}
impl<State: GetState<Game>, Z: GetZone, F> StateFilter<State, (ValidPlayerID<F>, ValidCardID<()>)>
    for CardIn<Z>
{
    type ValidOutput = (ValidPlayerID<F>, ValidCardID<Self>);
    type Error = CardDoesNotExist;
    fn filter(
        state: &State,
        (valid_player_id, valid_card_id): (ValidPlayerID<F>, ValidCardID<()>),
    ) -> Result<Self::ValidOutput, Self::Error> {
        let valid_card_id = ValidCardID::try_new(
            valid_card_id.id(),
            Z::get_zone(state.state(), &valid_player_id),
        )?;
        Ok((valid_player_id, valid_card_id))
    }
}
impl<State: GetState<Game>, Z: GetZone, F>
    StateFilter<State, FilterInput<(ValidPlayerID<F>, ValidCardID<()>)>> for CardIn<Z>
{
    type ValidOutput = FilterInput<(ValidPlayerID<F>, ValidCardID<Self>)>;
    type Error = CardDoesNotExist;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, valid_card_id)): FilterInput<(
            ValidPlayerID<F>,
            ValidCardID<()>,
        )>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        Self::filter(state, (valid_player_id, valid_card_id)).map(FilterInput)
    }
}

//Condition<FilterInput<(ValidPlayerID<ActivePlayer>, Tribute)>, CardIn<MonsterZone>>,
impl<State: GetState<Game>, F> StateFilter<State, FilterInput<(ValidPlayerID<F>, Tribute)>>
    for CardIn<MonsterZone>
{
    type ValidOutput = FilterInput<(ValidPlayerID<F>, ValidTribute)>;
    type Error = CardDoesNotExist;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, tribute)): FilterInput<(ValidPlayerID<F>, Tribute)>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        match ValidCardID::try_new(
            tribute.0,
            &state
                .state()
                .zone_manager()
                .valid_zone(&valid_player_id)
                .monster_zone,
        ) {
            Ok(valid_card_id) => Ok(FilterInput((valid_player_id, ValidTribute(valid_card_id)))),
            Err(e) => Err(e),
        }
    }
}
