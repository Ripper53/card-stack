use crate::{
    cards::CardID,
    identifications::PlayerID,
    zones::{Zone, ZoneContext},
};

pub struct ValidCardID<F>(CardID, std::marker::PhantomData<(F, *const ())>);
impl<F> ValidCardID<F> {
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
impl<F> From<ValidCardID<F>> for CardID {
    fn from(valid_card_id: ValidCardID<F>) -> Self {
        valid_card_id.0
    }
}
impl<F0, F1> From<ValidCardID<(F0, F1)>> for ValidCardID<F0> {
    fn from(valid_card_id: ValidCardID<(F0, F1)>) -> Self {
        ValidCardID::new(valid_card_id.0)
    }
}
macro_rules! impl_from_valid_card_id {
    ($($t: ident,)+ | $last_t: ident) => {
        impl<$($t,)*$last_t> From<ValidCardID<($($t,)*$last_t)>> for ValidCardID<($($t,)*)> {
            fn from(valid_card_id: ValidCardID<($($t,)*$last_t)>) -> Self {
                ValidCardID::new(valid_card_id.0)
            }
        }
    };
}
impl_from_valid_card_id!(F0, | F1);
impl_from_valid_card_id!(F0, F1, | F2);
impl_from_valid_card_id!(F0, F1, F2, | F3);

impl<Z> std::fmt::Debug for ValidCardID<Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidCardID({})", self.0)
    }
}
impl<Z> PartialEq for ValidCardID<Z> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<Z> Eq for ValidCardID<Z> {}
impl<Z> std::hash::Hash for ValidCardID<Z> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
