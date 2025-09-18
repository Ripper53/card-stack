use card_game::{
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::MonsterCard,
    filters::{CardIn, Free, In, MonsterSlot, OfType},
    identifications::ValidSlotID,
    zones::{GetZone, hand::HandZone},
};

pub struct With<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, Z: GetZone> StateFilter<State> for With<(Free<MonsterSlot>, In<Z>)> {
    type Input = (usize, PlayerID);
    type ValidOutput = (usize, ValidPlayerID<()>);
    fn filter(state: &State, _: Self::Input) -> Option<Self::ValidOutput> {
        //ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_index)
        //.map(|valid_slot_id| (valid_player_id, c, valid_slot_id))
        None
    }
}

impl<State: GetState<Game>> StateFilter<State> for With<MonsterSlot> {
    type Input = usize;
    type ValidOutput = ValidSlotID<Free<MonsterSlot>>;
    fn filter(state: &State, valid_player_id: Self::Input) -> Option<Self::ValidOutput> {
        //ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_index)
        //.map(|valid_slot_id| (valid_player_id, c, valid_slot_id))
        None
    }
}
