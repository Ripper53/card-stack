use crate::{
    cards::CardID,
    zones::{Zone, ZoneContext},
};

pub struct ZoneCardID<'a, Z: Zone>(CardID, std::marker::PhantomData<&'a Z>);
impl<'a, Z: Zone> ZoneCardID<'a, Z> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ZoneCardID(card_id, std::marker::PhantomData::default())
    }
    pub(crate) fn clone_id(&self) -> Self {
        ZoneCardID::new(self.0.clone_id())
    }
    pub fn card_id(&self) -> CardID {
        self.0
    }
    pub fn remove<T>(&self, f: impl FnOnce(&Self) -> Option<T>) -> T {
        f(self).unwrap()
    }
}
pub struct ValidZoneCardContext<State, Z> {
    card_id: CardID,
    state: State,
    _p: std::marker::PhantomData<Z>,
}
impl<State, Z: Zone> ValidZoneCardContext<State, Z> {
    pub fn new(
        state: State,
        get_zone: for<'a> fn(&'a State) -> &'a Z,
        get_valid_card_id: impl for<'a> FnOnce(ZoneContext<'a, Z>) -> ZoneCardID<'a, Z>,
    ) -> Self {
        let zone = ZoneContext::new(get_zone(&state));
        let card_id = get_valid_card_id(zone).card_id();
        ValidZoneCardContext {
            card_id,
            state,
            _p: std::marker::PhantomData::default(),
        }
    }
    pub fn execute<'a, R>(self, f: impl FnOnce(State, ZoneCardID<'a, Z>) -> R) -> R
    where
        Z: 'a,
    {
        f(self.state, ZoneCardID::new(self.card_id))
    }
}
impl<'a, Z: Zone> std::fmt::Debug for ZoneCardID<'a, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZoneCardID({})", self.0)
    }
}
impl<'a, Z: Zone> PartialEq for ZoneCardID<'a, Z> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<'a, Z: Zone> Eq for ZoneCardID<'a, Z> {}
impl<'a, Z: Zone> std::hash::Hash for ZoneCardID<'a, Z> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
