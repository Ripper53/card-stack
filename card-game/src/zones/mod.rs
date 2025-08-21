use std::collections::HashMap;

use crate::{
    cards::{Card, CardID},
    identifications::{PlayerID, PlayerManager},
};

mod context;
mod slot;
mod zone_card_id;
pub use context::*;
pub use slot::*;
pub use zone_card_id::*;

pub struct ZoneManager<Z: Zones> {
    zones: HashMap<PlayerID, Z>,
}

pub trait Zones {
    fn new(player_id: PlayerID) -> Self;
}

impl<Z: Zones> ZoneManager<Z> {
    pub fn new<P>(player_manager: &PlayerManager<P>) -> Self {
        let mut zones = HashMap::with_capacity(player_manager.players.len());
        for player_id in player_manager.players.keys().copied() {
            zones.insert(player_id, Z::new(player_id));
        }
        ZoneManager { zones }
    }
    pub fn get_zone(&self, player_id: PlayerID) -> Option<&Z> {
        self.zones.get(&player_id)
    }
    pub fn get_zone_mut(&mut self, player_id: PlayerID) -> Option<&mut Z> {
        self.zones.get_mut(&player_id)
    }
}

pub trait Zone: Sized {
    type CardKind;
    fn filled_count(&self) -> usize;
    fn get_card(&self, card_id: CardID) -> Option<&Card<Self::CardKind>>;
    fn get_card_from_index(&self, index: usize) -> Option<&Card<Self::CardKind>>;
    fn cards(&self) -> impl Iterator<Item = &Card<Self::CardKind>>;
}
pub trait FiniteZone: ArrayZone {
    fn max_count(&self) -> usize;
    fn has_space(&self) -> bool {
        self.max_count() - self.filled_count() != 0
    }
    fn add_card(&mut self, card: Card<Self::CardKind>) -> Result<(), ZoneFullError> {
        if self.has_space() {
            Ok(self.add_card_unchecked(card))
        } else {
            Err(ZoneFullError::new(card.id()))
        }
    }
    /// Use [`add_card`](Self::add_card) instead. Never call this method directly.
    fn add_card_unchecked(&mut self, card: Card<Self::CardKind>);
}
pub trait InfiniteZone: ArrayZone {
    fn add_card(&mut self, card: Card<Self::CardKind>);
}
pub trait ArrayZone: Zone {
    fn remove_card<'id>(&mut self, zone_card_id: ZoneCardID<'id, Self>) -> Card<Self::CardKind>;
}

#[derive(thiserror::Error, Debug)]
#[error("failed to add card to zone")]
pub struct ZoneFullError(CardID);

impl ZoneFullError {
    pub(crate) fn new(card_id: CardID) -> Self {
        ZoneFullError(card_id)
    }
    pub fn card_id(self) -> CardID {
        self.0
    }
}

pub struct ZoneToSlotTransport<'a, FromZ: ArrayZone, ToCardKind> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut Slot<ToCardKind>,
}

impl<'a, FromZ: ArrayZone, ToCardKind> ZoneToSlotTransport<'a, FromZ, ToCardKind>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<ToCardKind>>,
{
    pub fn transport(mut self, zone_card_id: ZoneCardID<'a, FromZ>) -> Result<(), ZoneFullError> {
        if self.to_zone.is_occupied() {
            Err(ZoneFullError(zone_card_id.card_id()))
        } else {
            let card = self.from_zone.remove_card(zone_card_id);
            let _ = self.to_zone.put(card.into());
            Ok(())
        }
    }
}
pub struct ZoneToFiniteZoneTransport<'a, FromZ: ArrayZone, ToZ: FiniteZone> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut ToZ,
}
impl<'a, FromZ: ArrayZone, ToZ: FiniteZone> ZoneToFiniteZoneTransport<'a, FromZ, ToZ>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<<ToZ as Zone>::CardKind>>,
{
    pub fn transport(mut self, zone_card_id: ZoneCardID<'a, FromZ>) -> Result<(), ZoneFullError> {
        if self.to_zone.has_space() {
            let card = self.from_zone.remove_card(zone_card_id);
            self.to_zone.add_card_unchecked(card.into());
            Ok(())
        } else {
            Err(ZoneFullError(zone_card_id.card_id()))
        }
    }
}

pub struct ZoneToInfiniteZoneTransport<'a, FromZ: ArrayZone, ToZ: InfiniteZone> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut ToZ,
}
impl<'a, FromZ: ArrayZone, ToZ: InfiniteZone> ZoneToInfiniteZoneTransport<'a, FromZ, ToZ>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<<ToZ as Zone>::CardKind>>,
{
    pub fn transport(mut self, zone_card_id: ZoneCardID<'a, FromZ>) {
        let card = self.from_zone.remove_card(zone_card_id);
        self.to_zone.add_card(card.into());
    }
}
