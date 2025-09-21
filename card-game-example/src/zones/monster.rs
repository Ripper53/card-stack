use card_game::{
    cards::Card,
    define_slot_iter,
    identifications::PlayerID,
    zones::{Slot, SlotZone, Zone},
};

use crate::{cards::monster::MonsterZoneCard, filters::CardIn, zones::GetZone};

pub struct MonsterZone {
    player_id: PlayerID,
    slot_a: Slot<MonsterZoneCard>,
    slot_b: Slot<MonsterZoneCard>,
    slot_c: Slot<MonsterZoneCard>,
    slot_d: Slot<MonsterZoneCard>,
    slot_e: Slot<MonsterZoneCard>,
}

impl MonsterZone {
    pub fn new(player_id: PlayerID) -> Self {
        MonsterZone {
            player_id,
            slot_a: Slot::new(),
            slot_b: Slot::new(),
            slot_c: Slot::new(),
            slot_d: Slot::new(),
            slot_e: Slot::new(),
        }
    }
    pub fn slot_a(&self) -> &Slot<MonsterZoneCard> {
        &self.slot_a
    }
    pub fn slot_b(&self) -> &Slot<MonsterZoneCard> {
        &self.slot_b
    }
    pub fn slot_c(&self) -> &Slot<MonsterZoneCard> {
        &self.slot_c
    }
    pub fn slot_d(&self) -> &Slot<MonsterZoneCard> {
        &self.slot_d
    }
    pub fn slot_e(&self) -> &Slot<MonsterZoneCard> {
        &self.slot_e
    }
    pub fn slot_a_mut(&mut self) -> &mut Slot<MonsterZoneCard> {
        &mut self.slot_a
    }
    pub fn slot_b_mut(&mut self) -> &mut Slot<MonsterZoneCard> {
        &mut self.slot_b
    }
    pub fn slot_c_mut(&mut self) -> &mut Slot<MonsterZoneCard> {
        &mut self.slot_c
    }
    pub fn slot_d_mut(&mut self) -> &mut Slot<MonsterZoneCard> {
        &mut self.slot_d
    }
    pub fn slot_e_mut(&mut self) -> &mut Slot<MonsterZoneCard> {
        &mut self.slot_e
    }
}

impl SlotZone for MonsterZone {
    fn max_slots(&self) -> usize {
        5
    }
}
impl Zone for MonsterZone {
    type CardKind = MonsterZoneCard;
    type CardFilter = CardIn<Self>;
    fn player_id(&self) -> PlayerID {
        self.player_id
    }
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
            MonsterZoneCard,
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

impl GetZone for MonsterZone {
    fn get_zone<'a, F>(
        game: &'a crate::Game,
        player_id: &'a card_game::identifications::ValidPlayerID<F>,
    ) -> &'a Self {
        &game.zone_manager().valid_zone(player_id).monster_zone
    }
}
