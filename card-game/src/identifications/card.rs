use state_validation::StateFilterInput;

use crate::{cards::CardID, create_valid_identification, zones::Zone};

use crate as card_game;
create_valid_identification!(ValidCardID, CardID);
impl<F> ValidCardID<F> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ValidCardID(card_id, std::marker::PhantomData::default())
    }
}
pub trait GetValidCardIDFromZone<Z: Zone>: Sized {
    fn try_new(card_id: CardID, zone: &Z) -> Result<Self, CardDoesNotExist>;
}
impl<Z: Zone> GetValidCardIDFromZone<Z> for ValidCardID<Z::CardFilter> {
    fn try_new(card_id: CardID, zone: &Z) -> Result<Self, CardDoesNotExist> {
        if zone.get_card(card_id).is_some() {
            Ok(ValidCardID::new(card_id))
        } else {
            Err(CardDoesNotExist(card_id))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("card {0} does not exist")]
pub struct CardDoesNotExist(pub CardID);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct SourceCardID(pub CardID);
impl std::fmt::Display for SourceCardID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct TargetCardID(pub CardID);
impl std::fmt::Display for TargetCardID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
