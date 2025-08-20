use card_game::{
    cards::Card,
    zones::{
        Zone,
        slot::{Slot, SlotZone},
    },
};

use crate::cards::monster::MonsterCard;

pub struct MonsterZone<'a> {
    slot_a: Slot<'a, Self, Card<MonsterCard>>,
    slot_b: Slot<'a, Self, Card<MonsterCard>>,
    slot_c: Slot<'a, Self, Card<MonsterCard>>,
    slot_d: Slot<'a, Self, Card<MonsterCard>>,
    slot_e: Slot<'a, Self, Card<MonsterCard>>,
}

impl<'a> MonsterZone<'a> {
    pub fn new() -> Self {
        MonsterZone {
            slot_a: Slot::new(),
            slot_b: Slot::new(),
            slot_c: Slot::new(),
            slot_d: Slot::new(),
            slot_e: Slot::new(),
        }
    }
    pub fn slot_a(&self) -> &Slot<'a, Self, Card<MonsterCard>> {
        &self.slot_a
    }
    pub fn slot_b(&self) -> &Slot<'a, Self, Card<MonsterCard>> {
        &self.slot_b
    }
    pub fn slot_c(&self) -> &Slot<'a, Self, Card<MonsterCard>> {
        &self.slot_c
    }
    pub fn slot_d(&self) -> &Slot<'a, Self, Card<MonsterCard>> {
        &self.slot_d
    }
    pub fn slot_e(&self) -> &Slot<'a, Self, Card<MonsterCard>> {
        &self.slot_e
    }
    pub fn slot_a_mut(&mut self) -> &mut Slot<'a, Self, Card<MonsterCard>> {
        &mut self.slot_a
    }
    pub fn slot_b_mut(&mut self) -> &mut Slot<'a, Self, Card<MonsterCard>> {
        &mut self.slot_b
    }
    pub fn slot_c_mut(&mut self) -> &mut Slot<'a, Self, Card<MonsterCard>> {
        &mut self.slot_c
    }
    pub fn slot_d_mut(&mut self) -> &mut Slot<'a, Self, Card<MonsterCard>> {
        &mut self.slot_d
    }
    pub fn slot_e_mut(&mut self) -> &mut Slot<'a, Self, Card<MonsterCard>> {
        &mut self.slot_e
    }
}

impl<'a> SlotZone for MonsterZone<'a> {
    fn max_slots(&self) -> usize {
        5
    }
}
impl<'a> Zone for MonsterZone<'a> {
    type CardKind = MonsterCard;
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
