use crate::{cards::CardID, create_valid_identification, identifications::PlayerID, zones::Zone};

use crate as card_game;
create_valid_identification!(ValidCardID, CardID);
impl<F> ValidCardID<F> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ValidCardID(card_id, std::marker::PhantomData::default())
    }
}
pub trait GetValidCardIDFromZone<Z: Zone>: Sized {
    fn try_new(card_id: CardID, zone: &Z) -> Option<Self>;
}
impl<Z: Zone> GetValidCardIDFromZone<Z> for ValidCardID<Z::CardFilter> {
    fn try_new(card_id: CardID, zone: &Z) -> Option<Self> {
        if zone.get_card(card_id).is_some() {
            Some(ValidCardID(card_id, std::marker::PhantomData::default()))
        } else {
            None
        }
    }
}
