use crate::{
    cards::CardID,
    zones::{ValidCardID, Zone},
};

pub struct ZoneContext<'a, Z: Zone, T> {
    zone: &'a Z,
    _m: std::marker::PhantomData<T>,
}

macro_rules! impl_zone_context {
    ($($t: ident,)*) => {
        impl<'a, Z: Zone + 'static $(, $t: 'static)*> ZoneContext<'a, Z, ($($t,)*)> {
            /// Should *NEVER* be created outside the crate
            /// *MUST* follow soundness, used only with [`ValidZoneCardContext`](crate::zones::ValidZoneCardContext).
            pub(crate) fn new(zone: &'a Z) -> Self {
                ZoneContext {
                    zone,
                    _m: std::marker::PhantomData::default(),
                }
            }
            pub fn get_zone_card_id(&self, card_id: CardID) -> Option<ValidCardID<'a, (Z $(, $t)*)>> {
                self.zone
                    .get_card(card_id)
                    .map(|card| ValidCardID::new(card.id()))
            }
            pub fn get_zone_card_id_from_index(&self, index: usize) -> Option<ValidCardID<'a, (Z $(, $t)*)>> {
                self.zone
                    .get_card_from_index(index)
                    .map(|card| ValidCardID::new(card.id()))
            }
            pub fn zone_card_ids(&self) -> impl Iterator<Item = ValidCardID<'a, (Z $(, $t)*)>> {
                self.zone.cards().map(|card| ValidCardID::new(card.id()))
            }
        }
    };
}
impl_zone_context!();
impl_zone_context!(T0,);
impl_zone_context!(T0, T1,);
impl_zone_context!(T0, T1, T2,);
impl_zone_context!(T0, T1, T2, T3,);
