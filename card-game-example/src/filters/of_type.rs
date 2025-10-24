use card_game::{
    cards::CardID,
    identifications::{ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
    zones::Zone,
};

use crate::{
    Game,
    cards::{
        CardKind,
        monster::{MonsterCard, MonsterCardType},
    },
    filters::{CardIn, FilterInput},
    steps::MainStep,
    zones::{GetZone, hand::HandZone, monster::MonsterZone},
};

pub struct OfType<T>(std::marker::PhantomData<T>);

#[derive(thiserror::Error, Debug)]
#[error("card {0} does not exist")]
pub struct CardIsNotMonsterError(CardID);

impl<State: GetState<Game>, F> StateFilter<State, (ValidPlayerID<F>, ValidCardID<CardIn<HandZone>>)>
    for OfType<MonsterCard>
{
    type ValidOutput = (ValidPlayerID<F>, ValidCardID<(CardIn<HandZone>, Self)>);
    type Error = CardIsNotMonsterError;
    fn filter(
        state: &State,
        (valid_player_id, valid_card_id): (ValidPlayerID<F>, ValidCardID<CardIn<HandZone>>),
    ) -> Result<Self::ValidOutput, Self::Error> {
        let card = state
            .state()
            .zone_manager()
            .valid_zone(&valid_player_id)
            .hand_zone()
            .valid_card(&valid_card_id);
        if matches!(card.kind(), CardKind::Monster(MonsterCardType::Monster(_))) {
            Ok((valid_player_id, valid_card_id.unchecked_replace_filter()))
        } else {
            Err(CardIsNotMonsterError(valid_card_id.id()))
        }
    }
}
impl<State: GetState<Game>, F>
    StateFilter<State, FilterInput<(ValidPlayerID<F>, ValidCardID<CardIn<HandZone>>)>>
    for OfType<MonsterCard>
{
    type ValidOutput = FilterInput<(ValidPlayerID<F>, ValidCardID<(CardIn<HandZone>, Self)>)>;
    type Error = CardIsNotMonsterError;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, valid_card_id)): FilterInput<(
            ValidPlayerID<F>,
            ValidCardID<CardIn<HandZone>>,
        )>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        Self::filter(state, (valid_player_id, valid_card_id)).map(|v| FilterInput(v))
    }
}

impl<State: GetState<Game>, F>
    StateFilter<State, (ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>)>
    for OfType<MonsterCard>
{
    type ValidOutput = (ValidPlayerID<F>, ValidCardID<(CardIn<MonsterZone>, Self)>);
    type Error = CardIsNotMonsterError;
    fn filter(
        state: &State,
        (valid_player_id, valid_card_id): (ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>),
    ) -> Result<Self::ValidOutput, Self::Error> {
        let card = state
            .state()
            .zone_manager()
            .valid_zone(&valid_player_id)
            .monster_zone
            .valid_card(&valid_card_id);
        if matches!(card.kind().kind(), MonsterCardType::Monster(_)) {
            Ok((valid_player_id, valid_card_id.unchecked_replace_filter()))
        } else {
            Err(CardIsNotMonsterError(valid_card_id.id()))
        }
    }
}
impl<State: GetState<Game>, F>
    StateFilter<State, FilterInput<(ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>)>>
    for OfType<MonsterCard>
{
    type ValidOutput = FilterInput<(ValidPlayerID<F>, ValidCardID<(CardIn<MonsterZone>, Self)>)>;
    type Error = CardIsNotMonsterError;
    fn filter(
        state: &State,
        FilterInput((valid_player_id, valid_card_id)): FilterInput<(
            ValidPlayerID<F>,
            ValidCardID<CardIn<MonsterZone>>,
        )>,
    ) -> Result<Self::ValidOutput, Self::Error> {
        Self::filter(state, (valid_player_id, valid_card_id)).map(|v| FilterInput(v))
    }
}
