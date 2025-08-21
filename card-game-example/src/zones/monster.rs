use card_game::{
    cards::Card,
    define_slot_iter,
    zones::{Slot, SlotZone, Zone},
};

use crate::cards::monster::MonsterCard;

pub struct MonsterZone {
    slot_a: Slot<MonsterCard>,
    slot_b: Slot<MonsterCard>,
    slot_c: Slot<MonsterCard>,
    slot_d: Slot<MonsterCard>,
    slot_e: Slot<MonsterCard>,
}

impl MonsterZone {
    pub fn new() -> Self {
        MonsterZone {
            slot_a: Slot::new(),
            slot_b: Slot::new(),
            slot_c: Slot::new(),
            slot_d: Slot::new(),
            slot_e: Slot::new(),
        }
    }
    pub fn slot_a(&self) -> &Slot<MonsterCard> {
        &self.slot_a
    }
    pub fn slot_b(&self) -> &Slot<MonsterCard> {
        &self.slot_b
    }
    pub fn slot_c(&self) -> &Slot<MonsterCard> {
        &self.slot_c
    }
    pub fn slot_d(&self) -> &Slot<MonsterCard> {
        &self.slot_d
    }
    pub fn slot_e(&self) -> &Slot<MonsterCard> {
        &self.slot_e
    }
    pub fn slot_a_mut(&mut self) -> &mut Slot<MonsterCard> {
        &mut self.slot_a
    }
    pub fn slot_b_mut(&mut self) -> &mut Slot<MonsterCard> {
        &mut self.slot_b
    }
    pub fn slot_c_mut(&mut self) -> &mut Slot<MonsterCard> {
        &mut self.slot_c
    }
    pub fn slot_d_mut(&mut self) -> &mut Slot<MonsterCard> {
        &mut self.slot_d
    }
    pub fn slot_e_mut(&mut self) -> &mut Slot<MonsterCard> {
        &mut self.slot_e
    }
}

impl SlotZone for MonsterZone {
    fn max_slots(&self) -> usize {
        5
    }
}
impl Zone for MonsterZone {
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
    fn get_card(&self, card_id: card_game::cards::CardID) -> Option<&Card<Self::CardKind>> {
        todo!()
    }
    fn get_card_from_index(&self, index: usize) -> Option<&Card<Self::CardKind>> {
        match index {
            0 => self.slot_a.occupier(),
            1 => self.slot_b.occupier(),
            2 => self.slot_c.occupier(),
            3 => self.slot_d.occupier(),
            4 => self.slot_e.occupier(),
            _ => None,
        }
    }
    fn cards(&self) -> impl Iterator<Item = &Card<Self::CardKind>> {
        define_slot_iter!(
            I,
            MonsterZone,
            MonsterCard,
            0 => slot_a,
            1 => slot_b,
            2 => slot_c,
            3 => slot_d,
            4 => slot_e,
        );
        I {
            index: 0,
            zone: self,
        }
    }
}
