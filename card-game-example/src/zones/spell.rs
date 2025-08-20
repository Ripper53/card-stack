use card_game::{
    cards::Card,
    zones::{
        Zone,
        slot::{Slot, SlotZone},
    },
};

use crate::cards::spell::SpellCard;

pub struct SpellZone<'a> {
    slot_a: Slot<'a, Self, Card<SpellCard>>,
    slot_b: Slot<'a, Self, Card<SpellCard>>,
    slot_c: Slot<'a, Self, Card<SpellCard>>,
    slot_d: Slot<'a, Self, Card<SpellCard>>,
    slot_e: Slot<'a, Self, Card<SpellCard>>,
}

impl<'a> SpellZone<'a> {
    pub fn new() -> Self {
        SpellZone {
            slot_a: Slot::new(),
            slot_b: Slot::new(),
            slot_c: Slot::new(),
            slot_d: Slot::new(),
            slot_e: Slot::new(),
        }
    }
    pub fn slot_a(&self) -> &Slot<'a, Self, Card<SpellCard>> {
        &self.slot_a
    }
    pub fn slot_b(&self) -> &Slot<'a, Self, Card<SpellCard>> {
        &self.slot_b
    }
    pub fn slot_c(&self) -> &Slot<'a, Self, Card<SpellCard>> {
        &self.slot_c
    }
    pub fn slot_d(&self) -> &Slot<'a, Self, Card<SpellCard>> {
        &self.slot_d
    }
    pub fn slot_e(&self) -> &Slot<'a, Self, Card<SpellCard>> {
        &self.slot_e
    }
    pub fn slot_a_mut(&mut self) -> &mut Slot<'a, Self, Card<SpellCard>> {
        &mut self.slot_a
    }
    pub fn slot_b_mut(&mut self) -> &mut Slot<'a, Self, Card<SpellCard>> {
        &mut self.slot_b
    }
    pub fn slot_c_mut(&mut self) -> &mut Slot<'a, Self, Card<SpellCard>> {
        &mut self.slot_c
    }
    pub fn slot_d_mut(&mut self) -> &mut Slot<'a, Self, Card<SpellCard>> {
        &mut self.slot_d
    }
    pub fn slot_e_mut(&mut self) -> &mut Slot<'a, Self, Card<SpellCard>> {
        &mut self.slot_e
    }
}

impl<'a> SlotZone for SpellZone<'a> {
    fn max_slots(&self) -> usize {
        5
    }
}
impl<'a> Zone for SpellZone<'a> {
    type CardKind = SpellCard;
    fn filled_count(&self) -> usize {
        let mut count = 0;
        if self.slot_a.is_occupied() {
            count += 1;
        }
        if self.slot_b.is_occupied() {
            count += 1;
        }
        if self.slot_c.is_occupied() {
            count += 1;
        }
        if self.slot_d.is_occupied() {
            count += 1;
        }
        if self.slot_e.is_occupied() {
            count += 1;
        }
        count
    }
}
