use std::collections::HashMap;

use crate::{
    cards::{Card, CardID},
    identifications::{MutID, PlayerID, PlayerManager, ValidCardID, ValidPlayerID},
};

mod slot;
pub use slot::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
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
    pub fn zones(&self) -> impl Iterator<Item = (ValidPlayerID<()>, &Z)> {
        self.zones
            .iter()
            .map(|(player_id, zone)| (ValidPlayerID::new(*player_id), zone))
    }
    pub fn zones_mut(&mut self) -> impl Iterator<Item = (ValidPlayerID<()>, &mut Z)> {
        self.zones
            .iter_mut()
            .map(|(player_id, zone)| (ValidPlayerID::new(*player_id), zone))
    }
    pub fn get_zone(&self, player_id: PlayerID) -> Option<&Z> {
        self.zones.get(&player_id)
    }
    pub fn valid_zone<F>(&self, valid_player_id: &ValidPlayerID<F>) -> &Z {
        self.get_zone(valid_player_id.id()).unwrap()
    }
    pub fn get_zone_mut(&mut self, player_id: PlayerID) -> Option<&mut Z> {
        self.zones.get_mut(&player_id)
    }
    pub fn valid_zone_mut<F>(&mut self, valid_player_id: ValidPlayerID<F>) -> &mut Z {
        self.get_zone_mut(valid_player_id.id()).unwrap()
    }
}

pub trait Zone: Sized {
    type CardKind;
    type CardFilter;
    fn filled_count(&self) -> usize;
    fn get_card(&self, card_id: CardID) -> Option<&Card<Self::CardKind>>;
    fn get_card_mut(&mut self, card_id: MutID<CardID>) -> Option<&mut Card<Self::CardKind>>;
    fn valid_card(&self, valid_card_id: &ValidCardID<Self::CardFilter>) -> &Card<Self::CardKind> {
        self.get_card(valid_card_id.id()).unwrap()
    }
    fn valid_card_mut(
        &mut self,
        valid_card_id: MutID<ValidCardID<Self::CardFilter>>,
    ) -> &mut Card<Self::CardKind> {
        self.get_card_mut(MutID::new(valid_card_id.take_id().id()))
            .unwrap()
    }
    fn get_card_from_index(&self, index: usize) -> Option<&Card<Self::CardKind>>;
    fn cards(&self) -> impl Iterator<Item = &Card<Self::CardKind>>;
    fn valid_cards(
        &self,
    ) -> impl Iterator<Item = (ValidCardID<Self::CardFilter>, &Card<Self::CardKind>)> {
        self.cards().map(|card| (ValidCardID::new(card.id()), card))
    }
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
    fn remove_card(&mut self, zone_card_id: ValidCardID<Self::CardFilter>) -> Card<Self::CardKind>;
    fn transfer_to_slot<'a, ToCardKind>(
        &'a mut self,
        to_slot: &'a mut Slot<ToCardKind>,
    ) -> ZoneToSlotTransport<'a, Self, ToCardKind>
    where
        Card<Self::CardKind>: Into<Card<ToCardKind>>,
    {
        ZoneToSlotTransport {
            from_zone: self,
            to_slot,
        }
    }
    fn transfer_to_infinite_zone<'a, Zone: InfiniteZone<CardFilter = Self::CardFilter>>(
        &'a mut self,
        to_zone: &'a mut Zone,
    ) -> ZoneToInfiniteZoneTransport<'a, Self, Zone>
    where
        Card<Self::CardKind>: Into<Card<Zone::CardKind>>,
    {
        ZoneToInfiniteZoneTransport {
            from_zone: self,
            to_zone,
        }
    }
    fn transfer_to_finite_zone<'a, Zone: FiniteZone<CardFilter = Self::CardFilter>>(
        &'a mut self,
        to_zone: &'a mut Zone,
    ) -> ZoneToFiniteZoneTransport<'a, Self, Zone>
    where
        Card<Self::CardKind>: Into<Card<Zone::CardKind>>,
    {
        ZoneToFiniteZoneTransport {
            from_zone: self,
            to_zone,
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("failed to add card {0} to zone")]
pub struct ZoneFullError(CardID);

impl ZoneFullError {
    pub fn new(card_id: CardID) -> Self {
        ZoneFullError(card_id)
    }
    pub fn card_id(self) -> CardID {
        self.0
    }
}

pub struct ZoneToSlotTransport<'a, FromZ: ArrayZone, ToCardKind> {
    from_zone: &'a mut FromZ,
    to_slot: &'a mut Slot<ToCardKind>,
}

impl<'a, FromZ: ArrayZone, ToCardKind> ZoneToSlotTransport<'a, FromZ, ToCardKind>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<ToCardKind>>,
{
    pub fn transport(
        mut self,
        zone_card_id: ValidCardID<FromZ::CardFilter>,
    ) -> Result<(), ZoneFullError> {
        if self.to_slot.is_occupied() {
            Err(ZoneFullError(zone_card_id.id()))
        } else {
            let card = self.from_zone.remove_card(zone_card_id);
            let _ = self.to_slot.put(card.into());
            Ok(())
        }
    }
}
pub struct ZoneToFiniteZoneTransport<
    'a,
    FromZ: ArrayZone<CardFilter = ToZ::CardFilter>,
    ToZ: FiniteZone,
> {
    from_zone: &'a mut FromZ,
    to_zone: &'a mut ToZ,
}
impl<'a, FromZ: ArrayZone<CardFilter = ToZ::CardFilter>, ToZ: FiniteZone>
    ZoneToFiniteZoneTransport<'a, FromZ, ToZ>
where
    Card<<FromZ as Zone>::CardKind>: Into<Card<<ToZ as Zone>::CardKind>>,
{
    pub fn transport(
        mut self,
        zone_card_id: ValidCardID<FromZ::CardFilter>,
    ) -> Result<(), ZoneFullError> {
        if self.to_zone.has_space() {
            let card = self.from_zone.remove_card(zone_card_id);
            self.to_zone.add_card_unchecked(card.into());
            Ok(())
        } else {
            Err(ZoneFullError(zone_card_id.id()))
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
    pub fn transport(mut self, zone_card_id: ValidCardID<FromZ::CardFilter>) {
        let card = self.from_zone.remove_card(zone_card_id);
        self.to_zone.add_card(card.into());
    }
}
