use crate::{
    cards::CardID,
    identifications::PlayerID,
    zones::{Zone, ZoneContext},
};

pub struct ValidCardID<'a, Z>(CardID, std::marker::PhantomData<(&'a Z, *const ())>);
impl<'a, Z> ValidCardID<'a, Z> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ValidCardID(card_id, std::marker::PhantomData::default())
    }
    pub fn card_id(&self) -> CardID {
        self.0
    }
    pub(crate) fn clone_id(&self) -> Self {
        ValidCardID::new(self.0.clone_id())
    }
    pub fn remove<T>(&self, f: impl FnOnce(&Self) -> Option<T>) -> T {
        f(self).unwrap()
    }
}
macro_rules! impl_from_valid_card_id {
    ($($t: ident,)* | $last_t: ident) => {
        impl<'a, Z0 $(, $t)*, $last_t> From<ValidCardID<'a, (Z0 $(, $t)*, $last_t)>> for ValidCardID<'a, (Z0 $(,$t)*)> {
            fn from(valid_card_id: ValidCardID<'a, (Z0 $(, $t)*, $last_t)>) -> Self {
                ValidCardID::new(valid_card_id.0)
            }
        }
    };
}
impl_from_valid_card_id!(| Z1);
impl_from_valid_card_id!(Z1, | Z2);
impl_from_valid_card_id!(Z1, Z2, | Z3);
impl_from_valid_card_id!(Z1, Z2, Z3, | Z4);

impl<'a, Z> std::fmt::Debug for ValidCardID<'a, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidCardID({})", self.0)
    }
}
impl<'a, Z> PartialEq for ValidCardID<'a, Z> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<'a, Z> Eq for ValidCardID<'a, Z> {}
impl<'a, Z> std::hash::Hash for ValidCardID<'a, Z> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
