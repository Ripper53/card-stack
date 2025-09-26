use card_game::{
    StateFilterInput,
    cards::Card,
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
    impl_state_filter_combination,
    validation::{StateFilterCombination, StateFilterInput},
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::MonsterCard,
    filters::{CardIn, OfType},
    zones::{
        deck::DeckZone, graveyard::GraveyardZone, hand::HandZone, monster::MonsterZone,
        spell::SpellZone,
    },
};

pub mod deck;
pub mod graveyard;
pub mod hand;
pub mod monster;
pub mod spell;

pub struct Zones {
    pub(crate) monster_zone: MonsterZone,
    pub(crate) spell_zone: SpellZone,
    pub(crate) graveyard_zone: GraveyardZone,
    pub(crate) deck_zone: DeckZone,
    pub(crate) hand_zone: HandZone,
}

impl Zones {
    pub fn hand_zone(&self) -> &HandZone {
        &self.hand_zone
    }
    pub fn hand_zone_mut(&mut self) -> &mut HandZone {
        &mut self.hand_zone
    }
}

impl card_game::zones::Zones for Zones {
    fn new(player_id: PlayerID) -> Self {
        Zones {
            monster_zone: MonsterZone::new(player_id),
            spell_zone: SpellZone::new(player_id),
            graveyard_zone: GraveyardZone::new(player_id),
            deck_zone: DeckZone::new(player_id),
            hand_zone: HandZone::new(player_id),
        }
    }
}

pub trait GetZone: Zone<CardFilter = CardIn<Self>> {
    fn get_zone<'a, F>(game: &'a Game, player_id: &'a ValidPlayerID<F>) -> &'a Self;
}
pub trait ContainsMonsterCards: Zone<CardFilter = CardIn<Self>> {
    fn get_zone_mut<'a, F>(game: &'a mut Game, player_id: ValidPlayerID<F>) -> &'a mut Self;
    fn remove_monster_card(
        &mut self,
        zone_card_id: ValidCardID<(CardIn<Self>, OfType<MonsterCard>)>,
    ) -> Card<MonsterCard>;
}

#[derive(StateFilterInput, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct SlotID(usize);
impl SlotID {
    pub fn new(id: usize) -> Self {
        SlotID(id)
    }
    pub fn index(&self) -> usize {
        self.0
    }
}
impl std::fmt::Display for SlotID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl_state_filter_combination!(SlotID, 1, 8, T);
