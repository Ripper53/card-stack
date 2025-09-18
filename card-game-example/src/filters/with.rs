use card_game::{
    identifications::{ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::StateFilter,
    zones::Zone,
};

use crate::{
    Game,
    filters::{Free, In, MonsterSlot},
    identifications::ValidSlotID,
    zones::{GetZone, hand::HandZone},
};

pub struct With<T>(std::marker::PhantomData<T>);

impl<State: GetState<Game>, F0, F1, Z: GetZone>
    StateFilter<State, (ValidPlayerID<F0>, ValidCardID<F1>, usize)>
    for With<(Free<MonsterSlot>, In<Z>)>
{
    type ValidOutput = (
        ValidPlayerID<F0>,
        ValidCardID<F1>,
        ValidSlotID<Free<MonsterSlot>>,
    );
    fn filter(
        state: &State,
        (valid_player_id, valid_card_id, slot_index): (ValidPlayerID<F0>, ValidCardID<F1>, usize),
    ) -> Option<Self::ValidOutput> {
        ValidSlotID::try_new::<Z, _>(state.state(), &valid_player_id, slot_index)
            .map(|valid_slot_id| (valid_player_id, valid_card_id, valid_slot_id))
    }
}
