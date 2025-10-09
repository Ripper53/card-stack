use card_game::{
    StateFilterInput,
    cards::{Card, CardBuilder, CardID, SourceCardFilter},
    identifications::{ActivePlayer, PlayerID, SourceCardID, TargetCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{
        ActionID, Condition, StateFilterCombination, StateFilterInput, StateFilterInputConversion,
    },
};

use crate::{
    Game,
    cards::{
        Name,
        monster::{
            Attack, Defense, FusionMonsterCard, Level, LinkMonsterCard, MonsterCard,
            RitualMonsterCard, SynchroMonsterCard, XyzMonsterCard,
        },
        spell::SpellCard,
        trap::TrapCard,
    },
    events::EventManager,
    filters::{Any, FilterInput, For, In, StaticName, With},
    steps::MainStep,
    valid_actions::{SpecialSummon, SpecialSummonRequirement},
    zones::{SlotID, hand::HandZone, monster::MonsterZone},
};

pub trait BlueEyesWhiteDestinyConstructedDeck {
    fn blue_eyes_white_dragon(&mut self) -> Card<MonsterCard>;
    fn neo_kaiser_sea_horse(&mut self) -> Card<MonsterCard>;
    /*fn blue_eyes_alternative_white_dragon(&mut self) -> Card<MonsterCard>;
    fn blue_eyes_jet_dragon(&mut self) -> Card<MonsterCard>;
    fn blue_eyes_abyss_dragon(&mut self) -> Card<MonsterCard>;
    fn dragon_spirit_of_white(&mut self) -> Card<MonsterCard>;
    fn blue_eyes_chaos_max_dragon(&mut self) -> Card<RitualMonsterCard>;
    fn blue_eyes_chaos_dragon(&mut self) -> Card<RitualMonsterCard>;
    fn the_white_stone_of_legend(&mut self) -> Card<MonsterCard>;
    fn the_white_stone_of_ancients(&mut self) -> Card<MonsterCard>;
    fn sage_with_eyes_of_blue(&mut self) -> Card<MonsterCard>;
    fn master_with_eyes_of_blue(&mut self) -> Card<MonsterCard>;
    fn dictator_of_d(&mut self) -> Card<MonsterCard>;
    fn nibiru_the_primal_being(&mut self) -> Card<MonsterCard>;
    fn ash_blossom_and_joyous_spring(&mut self) -> Card<MonsterCard>;
    fn effect_veiler(&mut self) -> Card<MonsterCard>;
    fn roar_of_the_blue_eyed_dragons(&mut self) -> Card<SpellCard>;
    fn chaos_form(&mut self) -> Card<SpellCard>;
    fn ultimate_fusion(&mut self) -> Card<SpellCard>;
    fn the_melody_of_awakening_dragon(&mut self) -> Card<SpellCard>;
    fn mausoleum_of_white(&mut self) -> Card<SpellCard>;
    fn burst_stream_of_destruction(&mut self) -> Card<SpellCard>;
    fn trade_in(&mut self) -> Card<SpellCard>;
    fn called_by_the_grave(&mut self) -> Card<SpellCard>;
    fn majesty_of_the_white_dragon(&mut self) -> Card<TrapCard>;
    fn true_light(&mut self) -> Card<TrapCard>;
    fn the_ultimate_creature_of_destruction(&mut self) -> Card<TrapCard>;
    fn infinite_imperpanence(&mut self) -> Card<TrapCard>;
    fn indigo_eyes_silver_dragon(&mut self) -> Card<XyzMonsterCard>;
    fn spirit_with_eyes_of_blue(&mut self) -> Card<LinkMonsterCard>;
    fn blue_eyes_ultimate_dragon(&mut self) -> Card<FusionMonsterCard>;
    fn neo_blue_eyes_ultimate_dragon(&mut self) -> Card<FusionMonsterCard>;
    fn blue_eyes_twin_burst_dragon(&mut self) -> Card<FusionMonsterCard>;
    fn blue_eyes_tyrant_dragon(&mut self) -> Card<FusionMonsterCard>;
    fn blue_eyes_spirit_dragon(&mut self) -> Card<SynchroMonsterCard>;
    fn azure_eyes_silver_dragon(&mut self) -> Card<SynchroMonsterCard>;
    fn hieratic_seal_of_the_heavenly_spheres(&mut self) -> Card<LinkMonsterCard>;
    fn maiden_of_white(&mut self) -> Card<MonsterCard>;
    fn wishes_for_eyes_of_blue(&mut self) -> Card<SpellCard>;
    fn blue_eyes_ultimate_spirit_dragon(&mut self) -> Card<SynchroMonsterCard>;*/
}

impl<'a> BlueEyesWhiteDestinyConstructedDeck for CardBuilder<'a, EventManager> {
    fn blue_eyes_white_dragon(&mut self) -> Card<MonsterCard> {
        self.build(MonsterCard::new(
            Name::new("Blue-Eyes White Dragon".into()),
            Level::new(8),
            Attack::new(3000),
            Defense::new(2500),
        ))
        .finish()
    }
    fn neo_kaiser_sea_horse(&mut self) -> Card<MonsterCard> {
        self.build(MonsterCard::new(
            Name::new("Neo Kaiser Sea Horse".into()),
            Level::new(4),
            Attack::new(1700),
            Defense::new(1650),
        ))
        .with_action::<MainStep, NeoKaiserSeaHorseSpecialSummon, SpecialSummon>()
        .finish()
    }
}

#[derive(StateFilterInput)]
pub struct NeoKaiserSeaHorseSpecialSummon {
    pub player_id: PlayerID,
    pub source_card_id: SourceCardID,
    pub slot_id: SlotID,
}
impl StateFilterInputConversion<FilterInput<PlayerID>> for NeoKaiserSeaHorseSpecialSummon {
    type Remainder = FilterInput<(SourceCardID, SlotID)>;
    fn split_take(self) -> (FilterInput<PlayerID>, Self::Remainder) {
        (
            FilterInput(self.player_id),
            FilterInput((self.source_card_id, self.slot_id)),
        )
    }
}
impl StateFilterInputConversion<FilterInput<(PlayerID, SourceCardID)>>
    for NeoKaiserSeaHorseSpecialSummon
{
    type Remainder = FilterInput<SlotID>;
    fn split_take(self) -> (FilterInput<(PlayerID, SourceCardID)>, Self::Remainder) {
        (
            FilterInput((self.player_id, self.source_card_id)),
            FilterInput(self.slot_id),
        )
    }
}
impl StateFilterInputConversion<SourceCardID> for NeoKaiserSeaHorseSpecialSummon {
    type Remainder = FilterInput<(PlayerID, SlotID)>;
    fn split_take(self) -> (SourceCardID, Self::Remainder) {
        (
            self.source_card_id,
            FilterInput((self.player_id, self.slot_id)),
        )
    }
}
impl StateFilterCombination<FilterInput<(PlayerID, SlotID)>> for SourceCardID {
    type Combined = NeoKaiserSeaHorseSpecialSummon;
    fn combine(self, value: FilterInput<(PlayerID, SlotID)>) -> Self::Combined {
        NeoKaiserSeaHorseSpecialSummon {
            player_id: value.0.0,
            source_card_id: self,
            slot_id: value.0.1,
        }
    }
}
impl SpecialSummonRequirement<MainStep> for NeoKaiserSeaHorseSpecialSummon {
    type Filter = (
        Condition<Self, SourceCardFilter<SpecialSummon>>,
        Condition<FilterInput<PlayerID>, For<ActivePlayer>>,
        Condition<
            FilterInput<ValidPlayerID<ActivePlayer>>,
            Any<(With<BlueEyesWhiteDragonName>, In<MonsterZone>)>,
        >,
    );
    type Zone = HandZone;
    fn handle_summon(
        state: &mut MainStep,
        FilterInput((valid_player_id, card_id, slot_id)): <Self::Filter as card_game::validation::StateFilter<MainStep, Self>>::ValidOutput,
    ) -> (
        card_game::identifications::ValidPlayerID<()>,
        card_game::identifications::ValidCardID<(
            crate::filters::CardIn<Self::Zone>,
            crate::filters::OfType<MonsterCard>,
        )>,
        crate::identifications::ValidSlotID<crate::filters::In<crate::zones::monster::MonsterZone>>,
    ) {
        todo!("HANDLE NEO KAISER SEA HORSE SPECIAL SUMMON")
    }
    fn action_id() -> ActionID {
        ActionID::new("neo_kaiser_sea_horse_special_summon")
    }
}
pub struct BlueEyesWhiteDragonName;
impl StaticName for BlueEyesWhiteDragonName {
    fn name() -> &'static str {
        "Blue-Eyes White Dragon"
    }
}
