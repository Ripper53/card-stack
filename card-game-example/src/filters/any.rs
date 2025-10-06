use card_game::{
    identifications::{ActivePlayer, PlayerDoesNotExist, PlayerID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
};

use crate::{
    Game,
    cards::CardName,
    filters::{FilterInput, In, With},
    zones::GetZone,
};

pub struct Any<T>(std::marker::PhantomData<T>);

pub trait StaticName {
    fn name() -> &'static str;
}

impl<State: GetState<Game>, Z: GetZone, N: StaticName> StateFilter<State, FilterInput<PlayerID>>
    for Any<(With<N>, In<Z>)>
where
    Z::CardKind: CardName,
{
    type ValidOutput = FilterInput<ValidPlayerID<()>>;
    type Error = CardWithNameError;
    fn filter(
        state: &State,
        FilterInput(player_id): FilterInput<PlayerID>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        let valid_player_id: ValidPlayerID<()> =
            ValidPlayerID::try_new(state.state().player_manager(), player_id)?;
        if Z::get_zone(state.state(), &valid_player_id)
            .cards()
            .any(|card| card.name().contains(N::name()))
        {
            Ok(FilterInput(valid_player_id))
        } else {
            Err(CardWithNameNotFoundError.into())
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("card with name not found")]
pub struct CardWithNameNotFoundError;
#[derive(thiserror::Error, Debug)]
pub enum CardWithNameError {
    #[error(transparent)]
    Player(#[from] PlayerDoesNotExist),
    #[error(transparent)]
    NameNotFound(#[from] CardWithNameNotFoundError),
}

impl<State: GetState<Game>, Z: GetZone, N: StaticName, F>
    StateFilter<State, FilterInput<ValidPlayerID<F>>> for Any<(With<N>, In<Z>)>
where
    Z::CardKind: CardName,
{
    type ValidOutput = FilterInput<ValidPlayerID<F>>;
    type Error = CardWithNameNotFoundError;
    fn filter(
        state: &State,
        FilterInput(valid_player_id): FilterInput<ValidPlayerID<F>>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        if Z::get_zone(state.state(), &valid_player_id)
            .cards()
            .any(|card| card.name().contains(N::name()))
        {
            Ok(FilterInput(valid_player_id))
        } else {
            Err(CardWithNameNotFoundError)
        }
    }
}
