use card_game::{
    stack::priority::GetState,
    validation::StateFilter,
    zones::{ValidCardID, Zone},
};

use crate::{
    Game, cards::monster::MonsterCard, filters::CardIn, steps::MainStep, zones::hand::HandZone,
};

pub struct OfType<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>> StateFilter<State> for OfType<MonsterCard> {
    type Input = ValidCardID<CardIn<HandZone>>;
    type ValidOutput = ValidCardID<(CardIn<HandZone>, Self)>;
    fn filter(state: &State, value: Self::Input) -> Option<Self::ValidOutput> {
        todo!()
    }
}
