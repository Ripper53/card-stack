use card_game::{create_valid_identification, identifications::ValidPlayerID, zones::ZoneManager};

use crate::{
    Game,
    zones::{GetZone, SlotID, Zones},
};

create_valid_identification!(ValidSlotID, SlotID);
impl<F> ValidSlotID<F> {
    pub fn try_new<Z: GetZone, F0>(
        game: &Game,
        valid_player_id: &ValidPlayerID<F0>,
        slot_id: SlotID,
    ) -> Option<Self> {
        if Z::get_zone(game, valid_player_id)
            .get_card_from_index(slot_id.index())
            .is_none()
        {
            Some(ValidSlotID(slot_id, std::marker::PhantomData::default()))
        } else {
            None
        }
    }
    pub fn index(&self) -> usize {
        self.0.index()
    }
}
