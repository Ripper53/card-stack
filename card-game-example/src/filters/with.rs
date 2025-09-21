use card_game::{
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{StateFilter, StateFilterInputConversion},
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::MonsterCard,
    filters::{CardIn, Free, In, MonsterSlot, OfType},
    identifications::ValidSlotID,
    zones::{GetZone, SlotID, hand::HandZone},
};

pub struct With<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, Z: GetZone> StateFilter<State, PlayerID>
    for With<(Free<MonsterSlot>, In<Z>)>
{
    type ValidOutput = ValidPlayerID<()>;
    fn filter(state: &State, _: PlayerID) -> Option<Self::ValidOutput> {
        //ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_index)
        //.map(|valid_slot_id| (valid_player_id, c, valid_slot_id))
        None
    }
}

impl<State: GetState<Game>> StateFilter<State, SlotID> for With<MonsterSlot> {
    type ValidOutput = ValidSlotID<Free<MonsterSlot>>;
    fn filter(state: &State, valid_player_id: SlotID) -> Option<Self::ValidOutput> {
        //ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_index)
        //.map(|valid_slot_id| (valid_player_id, c, valid_slot_id))
        None
    }
}

/*impl StateFilterInput<SlotID> for (ValidPlayerID<()>, SlotID) {
    type Remainder = ValidPlayerID<()>;
    fn new(input: ValidPlayerID<()>, remainder: Self::Remainder) -> Self {
        todo!()
    }
    fn split_take(self) -> (SlotID, Self::Remainder) {
        todo!();
    }
}*/
