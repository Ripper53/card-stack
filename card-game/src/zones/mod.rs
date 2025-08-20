use std::collections::HashMap;

use crate::{
    cards::{Card, CardID},
    identifications::{PlayerID, PlayerManager},
    zones::slot::{Slot, SlotZone, ZoneSlotOccupiedError},
};

pub mod slot;

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

pub struct ZoneCardID<'a, Z: Zone>(CardID, std::marker::PhantomData<&'a Z>);
impl<'a, Z: Zone> ZoneCardID<'a, Z> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ZoneCardID(card_id, std::marker::PhantomData::default())
    }
    pub(crate) fn clone_id(&self) -> Self {
        ZoneCardID::new(self.0.clone_id())
    }
    pub fn remove<T>(&self, f: impl FnOnce(&Self) -> Option<T>) -> T {
        f(self).unwrap()
    }
}
impl<'a, Z: Zone> std::fmt::Debug for ZoneCardID<'a, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZoneCardID({})", self.0)
    }
}
impl<'a, Z: Zone> Clone for ZoneCardID<'a, Z> {
    fn clone(&self) -> Self {
        self.clone_id()
    }
}
impl<'a, Z: Zone> Copy for ZoneCardID<'a, Z> {}
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

pub trait Zone: Sized {
    type CardKind;
    fn filled_count(&self) -> usize;
}
pub trait FiniteZone<'a>: ArrayZone<'a>
where
    Self: 'a,
{
    fn max_count(&self) -> usize;
    fn has_space(&self) -> bool {
        self.max_count() - self.filled_count() != 0
    }
    fn add_card(&mut self, card: Card<Self::CardKind>) -> Result<(), ZoneFullError<'a, Self>> {
        if self.has_space() {
            Ok(self.add_card_unchecked(ZoneCardID::new(card.id().clone_id()), card))
        } else {
            Err(ZoneFullError::new(card.id().clone_id()))
        }
    }
    /// Use [`add_card`](Self::add_card) instead. Never call this method directly.
    fn add_card_unchecked(
        &mut self,
        zone_card_id: ZoneCardID<'a, Self>,
        card: Card<Self::CardKind>,
    );
}
pub trait InfiniteZone<'a>: ArrayZone<'a>
where
    Self: 'a,
{
    fn add_card(&mut self, card: Card<Self::CardKind>) {
        self.add_card_with_id(ZoneCardID::new(card.id().clone_id()), card);
    }
    /// Use [`add_card`](Self::add_card) instead. Never call this method directly.
    fn add_card_with_id(&mut self, zone_card_id: ZoneCardID<'a, Self>, card: Card<Self::CardKind>);
}
pub trait ArrayZone<'a>: Zone
where
    Self: 'a,
{
    fn remove_card(&mut self, zone_card_id: ZoneCardID<'a, Self>) -> Card<Self::CardKind>;
}

#[derive(thiserror::Error, Debug)]
#[error("failed to add card to zone")]
pub struct ZoneFullError<'a, Z: Zone>(ZoneCardID<'a, Z>);

impl<'a, Z: Zone> ZoneFullError<'a, Z> {
    pub(crate) fn new(card_id: CardID) -> Self {
        ZoneFullError(ZoneCardID::new(card_id))
    }
    pub fn take_card_id(self) -> ZoneCardID<'a, Z> {
        self.0
    }
}

pub struct ZoneToSlotTransport<'a, FromZ: ArrayZone<'a>, ToZ: SlotZone, ToCardKind> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut Slot<'a, ToZ, ToCardKind>,
}

impl<'a, FromZ: ArrayZone<'a>, ToZ: SlotZone, ToCardKind>
    ZoneToSlotTransport<'a, FromZ, ToZ, ToCardKind>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<ToCardKind>>,
{
    pub fn transport(mut self, zone_card_id: ZoneCardID<'a, FromZ>) -> Result<(), ZoneIsFullError> {
        if self.to_zone.is_occupied() {
            Err(ZoneIsFullError)
        } else {
            let card = self.from_zone.remove_card(zone_card_id);
            let _ = self.to_zone.put(card.into());
            Ok(())
        }
    }
}
pub struct ZoneToFiniteZoneTransport<'a, FromZ: ArrayZone<'a>, ToZ: FiniteZone<'a>> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut ToZ,
}
impl<'a, FromZ: ArrayZone<'a>, ToZ: FiniteZone<'a>> ZoneToFiniteZoneTransport<'a, FromZ, ToZ>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<<ToZ as Zone>::CardKind>>,
{
    pub fn transport(mut self, zone_card_id: ZoneCardID<'a, FromZ>) -> Result<(), ZoneIsFullError> {
        if self.to_zone.has_space() {
            let card_id = zone_card_id.0.clone_id();
            let card = self.from_zone.remove_card(zone_card_id);
            self.to_zone
                .add_card_unchecked(ZoneCardID::new(card_id), card.into());
            Ok(())
        } else {
            Err(ZoneIsFullError)
        }
    }
}

pub struct ZoneToInfiniteZoneTransport<'a, FromZ: ArrayZone<'a>, ToZ: InfiniteZone<'a>> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut ToZ,
}
impl<'a, FromZ: ArrayZone<'a>, ToZ: InfiniteZone<'a>> ZoneToInfiniteZoneTransport<'a, FromZ, ToZ>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<<ToZ as Zone>::CardKind>>,
{
    pub fn transport(mut self, zone_card_id: ZoneCardID<'a, FromZ>) {
        let card = self.from_zone.remove_card(zone_card_id);
        self.to_zone.add_card(card.into());
    }
}

#[derive(thiserror::Error, Debug)]
#[error("zone is full")]
pub struct ZoneIsFullError;
