use crate::{
    cards::CardID,
    zones::{Zone, ValidCardID},
};

pub struct ZoneContext<'a, Z: Zone> {
    zone: &'a Z,
}

impl<'a, Z: Zone> ZoneContext<'a, Z> {
    /// Should *NEVER* be created outside the crate
    /// *MUST* follow soundness, used only with [`ValidZoneCardContext`](crate::zones::ValidZoneCardContext).
    pub(crate) fn new(zone: &'a Z) -> Self {
        ZoneContext { zone }
    }
    pub fn get_zone_card_id(&self, card_id: CardID) -> Option<ValidCardID<'a, Z>> {
        self.zone
            .get_card(card_id)
            .map(|card| ValidCardID::new(card.id()))
    }
    pub fn get_zone_card_id_from_index(&self, index: usize) -> Option<ValidCardID<'a, Z>> {
        self.zone
            .get_card_from_index(index)
            .map(|card| ValidCardID::new(card.id()))
    }
    pub fn zone_card_ids(&self) -> impl Iterator<Item = ValidCardID<'a, Z>> {
        self.zone.cards().map(|card| ValidCardID::new(card.id()))
    }
}
