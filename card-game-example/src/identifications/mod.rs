use card_game::{create_valid_identification, identifications::ValidPlayerID, zones::ZoneManager};

use crate::{
    Game,
    zones::{GetZone, SlotID, Zones},
};

create_valid_identification!(ValidSlotID, SlotID, with_copy);
impl<F> ValidSlotID<F> {
    pub fn try_new<Z: GetZone, F0>(
        game: &Game,
        valid_player_id: &ValidPlayerID<F0>,
        slot_id: SlotID,
    ) -> Result<Self, SlotDoesNotExistError> {
        if Z::get_zone(game, valid_player_id)
            .get_card_from_index(slot_id.index())
            .is_none()
        {
            Ok(ValidSlotID(slot_id, std::marker::PhantomData::default()))
        } else {
            Err(SlotDoesNotExistError(slot_id))
        }
    }
    pub fn index(&self) -> usize {
        self.0.index()
    }
}
#[derive(thiserror::Error, Debug)]
#[error("slot {0} does not exist")]
pub struct SlotDoesNotExistError(SlotID);
