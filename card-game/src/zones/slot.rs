use crate::{
    cards::{Card, CardID},
    zones::{ArrayZone, FiniteZone, InfiniteZone, Zone, ZoneCardID},
};

pub trait SlotZone: Zone {
    fn max_slots(&self) -> usize;
}
pub struct Slot<'a, Z: SlotZone, CardKind> {
    card: Option<(ZoneCardID<'a, Z>, Card<CardKind>)>,
}
impl<'a, Z: SlotZone, CardKind> Slot<'a, Z, CardKind> {
    pub fn new() -> Self {
        Slot { card: None }
    }
    pub fn is_occupied(&self) -> bool {
        self.card.is_some()
    }
    pub fn put(&mut self, card: Card<CardKind>) -> Option<Card<CardKind>> {
        self.card
            .replace((ZoneCardID::new(card.id().clone_id()), card))
            .map(|(_, card)| card)
    }
    pub fn card_id(&self) -> Option<ZoneCardID<'a, Z>> {
        self.card.as_ref().map(|(id, _)| id.clone_id())
    }
    pub fn occupier(&self) -> Option<(ZoneCardID<'a, Z>, &Card<CardKind>)> {
        self.card.as_ref().map(|(id, card)| (id.clone_id(), card))
    }
    pub fn transfer_to_slot<ToZ: SlotZone, ToCardKind>(
        &'a mut self,
        to_slot: &'a mut Slot<'a, ToZ, ToCardKind>,
    ) -> SlotToSlotTransport<'a, Z, ToZ, CardKind, ToCardKind>
    where
        Card<CardKind>: Into<Card<ToCardKind>>,
    {
        SlotToSlotTransport {
            from_slot: self,
            to_slot,
        }
    }
    pub fn transfer_to_infinite_zone<Zone: InfiniteZone<'a>>(
        &'a mut self,
        to_zone: &'a mut Zone,
    ) -> SlotToInfiniteZoneTransport<'a, Z, CardKind, Zone>
    where
        Card<CardKind>: Into<Card<Zone::CardKind>>,
    {
        SlotToInfiniteZoneTransport {
            from_slot: self,
            to_zone,
        }
    }
    pub fn transfer_to_finite_zone<Zone: FiniteZone<'a>>(
        &'a mut self,
        to_zone: &'a mut Zone,
    ) -> SlotToFiniteZoneTransport<'a, Z, CardKind, Zone>
    where
        Card<CardKind>: Into<Card<Zone::CardKind>>,
    {
        SlotToFiniteZoneTransport {
            from_slot: self,
            to_zone,
        }
    }
}

pub struct SlotToSlotTransport<'a, FromZ: SlotZone, ToZ: SlotZone, FromCardKind, ToCardKind>
where
    Card<FromCardKind>: Into<Card<ToCardKind>>,
{
    from_slot: &'a mut Slot<'a, FromZ, FromCardKind>,
    to_slot: &'a mut Slot<'a, ToZ, ToCardKind>,
}
pub struct SlotToFiniteZoneTransport<'a, Z: SlotZone, FromCardKind, ToZone: FiniteZone<'a>>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    from_slot: &'a mut Slot<'a, Z, FromCardKind>,
    to_zone: &'a mut ToZone,
}
pub struct SlotToInfiniteZoneTransport<'a, Z: SlotZone, FromCardKind, ToZone: InfiniteZone<'a>>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    from_slot: &'a mut Slot<'a, Z, FromCardKind>,
    to_zone: &'a mut ToZone,
}

impl<'a, FromZ: SlotZone, ToZ: SlotZone, FromCardKind, ToCardKind>
    SlotToSlotTransport<'a, FromZ, ToZ, FromCardKind, ToCardKind>
where
    Card<FromCardKind>: Into<Card<ToCardKind>>,
{
    pub fn transport(mut self) -> Result<(), SlotToSlotTransportError> {
        if self.to_slot.is_occupied() {
            Err(SlotToSlotTransportError::SlotOccupied(
                ZoneSlotOccupiedError,
            ))
        } else if let Some((_id, card)) = self.from_slot.card.take() {
            self.to_slot.card = Some((ZoneCardID::new(card.id().clone_id()), card.into()));
            Ok(())
        } else {
            Err(SlotToSlotTransportError::NoSlotCard(CardNotInSlotError))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("slot is occupied already")]
pub struct ZoneSlotOccupiedError;
#[derive(thiserror::Error, Debug)]
#[error("no card in slot to transport")]
pub struct CardNotInSlotError;
#[derive(thiserror::Error, Debug)]
pub enum SlotToSlotTransportError {
    #[error(transparent)]
    SlotOccupied(#[from] ZoneSlotOccupiedError),
    #[error(transparent)]
    NoSlotCard(#[from] CardNotInSlotError),
}

impl<'a, Z: SlotZone, FromCardKind, ToZone: FiniteZone<'a>>
    SlotToFiniteZoneTransport<'a, Z, FromCardKind, ToZone>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    pub fn transport(mut self) -> Result<(), SlotToFiniteZoneTransportError> {
        if self.to_zone.has_space() {
            if let Some((zone_card_id, card)) = self.from_slot.card.take() {
                self.to_zone
                    .add_card_unchecked(ZoneCardID::new(zone_card_id.0), card.into());
                Ok(())
            } else {
                Err(SlotToFiniteZoneTransportError::CardNotInSlot(
                    CardNotInSlotError,
                ))
            }
        } else {
            Err(SlotToFiniteZoneTransportError::ZoneIsFull)
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum SlotToFiniteZoneTransportError {
    #[error("zone is full")]
    ZoneIsFull,
    #[error(transparent)]
    CardNotInSlot(#[from] CardNotInSlotError),
}

impl<'a, FromZone: SlotZone, FromCardKind, ToZone: InfiniteZone<'a>>
    SlotToInfiniteZoneTransport<'a, FromZone, FromCardKind, ToZone>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    pub fn transport(mut self) -> Result<(), CardNotInSlotError> {
        if let Some((_id, card)) = self.from_slot.card.take() {
            self.to_zone.add_card(card.into());
            Ok(())
        } else {
            Err(CardNotInSlotError)
        }
    }
}
