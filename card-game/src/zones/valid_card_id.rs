use crate::{
    cards::CardID,
    identifications::PlayerID,
    zones::{Zone, ZoneContext},
};

pub struct ValidCardID<'a, Z: Zone>(CardID, std::marker::PhantomData<(&'a Z, *const ())>);
impl<'a, Z: Zone> ValidCardID<'a, Z> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ValidCardID(card_id, std::marker::PhantomData::default())
    }
    pub(crate) fn clone_id(&self) -> Self {
        ValidCardID::new(self.0.clone_id())
    }
    pub fn card_id(&self) -> CardID {
        self.0
    }
    pub fn remove<T>(&self, f: impl FnOnce(&Self) -> Option<T>) -> T {
        f(self).unwrap()
    }
}

impl<'a, Z: Zone> std::fmt::Debug for ValidCardID<'a, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZoneCardID({})", self.0)
    }
}
impl<'a, Z: Zone> PartialEq for ValidCardID<'a, Z> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<'a, Z: Zone> Eq for ValidCardID<'a, Z> {}
impl<'a, Z: Zone> std::hash::Hash for ValidCardID<'a, Z> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
