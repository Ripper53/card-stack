use crate::{
    cards::Card,
    zones::{FiniteZone, InfiniteZone, Zone},
};

pub trait SlotZone: Zone {
    fn max_slots(&self) -> usize;
}
pub struct Slot<CardKind> {
    card: Option<Card<CardKind>>,
}
impl<CardKind> Slot<CardKind> {
    pub fn new() -> Self {
        Slot { card: None }
    }
    pub fn is_occupied(&self) -> bool {
        self.card.is_some()
    }
    pub fn put(&mut self, card: Card<CardKind>) -> Option<Card<CardKind>> {
        self.card.replace(card)
    }
    pub fn occupier(&self) -> Option<&Card<CardKind>> {
        self.card.as_ref()
    }
    pub fn occupier_mut(&mut self) -> Option<&mut Card<CardKind>> {
        self.card.as_mut()
    }
    pub fn transfer_to_slot<'a, ToCardKind>(
        &'a mut self,
        to_slot: &'a mut Slot<ToCardKind>,
    ) -> SlotToSlotTransport<'a, CardKind, ToCardKind>
    where
        Card<CardKind>: Into<Card<ToCardKind>>,
    {
        SlotToSlotTransport {
            from_slot: self,
            to_slot,
        }
    }
    pub fn transfer_to_infinite_zone<'a, Zone: InfiniteZone>(
        &'a mut self,
        to_zone: &'a mut Zone,
    ) -> SlotToInfiniteZoneTransport<'a, CardKind, Zone>
    where
        Card<CardKind>: Into<Card<Zone::CardKind>>,
    {
        SlotToInfiniteZoneTransport {
            from_slot: self,
            to_zone,
        }
    }
    pub fn transfer_to_finite_zone<'a, Zone: FiniteZone>(
        &'a mut self,
        to_zone: &'a mut Zone,
    ) -> SlotToFiniteZoneTransport<'a, CardKind, Zone>
    where
        Card<CardKind>: Into<Card<Zone::CardKind>>,
    {
        SlotToFiniteZoneTransport {
            from_slot: self,
            to_zone,
        }
    }
}

pub struct SlotToSlotTransport<'a, FromCardKind, ToCardKind>
where
    Card<FromCardKind>: Into<Card<ToCardKind>>,
{
    from_slot: &'a mut Slot<FromCardKind>,
    to_slot: &'a mut Slot<ToCardKind>,
}
pub struct SlotToFiniteZoneTransport<'a, FromCardKind, ToZone: FiniteZone>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    from_slot: &'a mut Slot<FromCardKind>,
    to_zone: &'a mut ToZone,
}
pub struct SlotToInfiniteZoneTransport<'a, FromCardKind, ToZone: InfiniteZone>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    from_slot: &'a mut Slot<FromCardKind>,
    to_zone: &'a mut ToZone,
}

impl<'a, FromCardKind, ToCardKind> SlotToSlotTransport<'a, FromCardKind, ToCardKind>
where
    Card<FromCardKind>: Into<Card<ToCardKind>>,
{
    pub fn transport(mut self) -> Result<(), SlotToSlotTransportError> {
        if self.to_slot.is_occupied() {
            Err(SlotToSlotTransportError::SlotOccupied(
                ZoneSlotOccupiedError,
            ))
        } else if let Some(card) = self.from_slot.card.take() {
            self.to_slot.card = Some(card.into());
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

impl<'a, FromCardKind, ToZone: FiniteZone> SlotToFiniteZoneTransport<'a, FromCardKind, ToZone>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    pub fn transport(mut self) -> Result<(), SlotToFiniteZoneTransportError> {
        if self.to_zone.has_space() {
            if let Some(card) = self.from_slot.card.take() {
                self.to_zone.add_card_unchecked(card.into());
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

impl<'a, FromCardKind, ToZone: InfiniteZone> SlotToInfiniteZoneTransport<'a, FromCardKind, ToZone>
where
    Card<FromCardKind>: Into<Card<<ToZone as Zone>::CardKind>>,
{
    pub fn transport(mut self) -> Result<(), CardNotInSlotError> {
        if let Some(card) = self.from_slot.card.take() {
            self.to_zone.add_card(card.into());
            Ok(())
        } else {
            Err(CardNotInSlotError)
        }
    }
}

#[macro_export]
macro_rules! define_slot_iter {
    ($iter_name: ident, $zone: ty, $card_type: ty, $($index: literal => $slots: ident,)+) => {
        struct $iter_name<'a> {
            index: usize,
            zone: &'a $zone,
        }
        impl<'a> ::std::iter::Iterator for $iter_name<'a> {
            type Item = &'a ::card_game::cards::Card<$card_type>;
            fn next(&mut self) -> ::std::option::Option<Self::Item> {
                loop {
                    match self.index {
                        $(
                            $index => {
                                self.index += 1;
                                if let Some(occupier) = self.zone.$slots.occupier() {
                                    break Some(occupier);
                                }
                            }
                        )*
                        _ => break None,
                    }
                }
            }
        }
    };
}
