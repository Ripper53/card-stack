use card_game::{
    cards::{Card, CardBuilder, CardID, SourceCardFilter},
    events::TriggeredEvent,
    identifications::{
        ActionID, ActionIdentifier, ActivePlayer, PlayerID, SourceCardID, TargetCardID,
        ValidCardID, ValidPlayerID,
    },
    stack::priority::GetState,
};
use state_validation::{
    Condition, StateFilterConversion, StateFilterInputCombination, StateFilterInputConversion,
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
    events::{
        EventManager,
        summon::{NormalSummoned, Summoned},
    },
    filters::{Any, CardIn, For, Free, In, MonsterSlot, OfType, StaticName, With},
    identifications::ValidSlotID,
    steps::MainStep,
    valid_actions::{GiveAttack, PassiveGiveAttack, SpecialSummon, SpecialSummonRequirement},
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
    fn alexandrite_dragon(&mut self) -> Card<MonsterCard>;
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
        .with_action::<SpecialSummon<MainStep, NeoKaiserSeaHorseSpecialSummon>>()
        .finish()
    }
    fn alexandrite_dragon(&mut self) -> Card<MonsterCard> {
        self.build(MonsterCard::new(
            Name::new("Alexandrite Dragon".into()),
            Level::new(4),
            Attack::new(2000),
            Defense::new(100),
        ))
        .finish()
    }
}

#[derive(StateFilterConversion)]
pub struct NeoKaiserSeaHorseSpecialSummon {
    #[conversion(T0 = ValidPlayerID<T0>)]
    pub player_id: PlayerID,
    #[conversion(T1 = ValidCardID<T1>)]
    pub source_card_id: SourceCardID,
    #[conversion(T2 = ValidSlotID<T2>)]
    pub slot_id: SlotID,
}
impl ActionIdentifier for NeoKaiserSeaHorseSpecialSummon {
    fn action_id() -> ActionID {
        ActionID::new("neo_kaiser_sea_horse_special_summon")
    }
}
impl SpecialSummonRequirement<MainStep> for NeoKaiserSeaHorseSpecialSummon {
    type Filter = (
        Condition<Self, SourceCardFilter<SpecialSummon<MainStep, Self>>>,
        Condition<PlayerID, For<ActivePlayer>>,
        Condition<(ValidPlayerID<ActivePlayer>, ValidCardID<()>), CardIn<Self::Zone>>,
        Condition<
            (ValidPlayerID<ActivePlayer>, ValidCardID<CardIn<Self::Zone>>),
            OfType<MonsterCard>,
        >,
        Condition<
            (ValidPlayerID<ActivePlayer>, SlotID),
            With<(Free<MonsterSlot>, In<MonsterZone>)>,
        >,
        /*Condition<
            FilterInput<ValidPlayerID<ActivePlayer>>,
            Any<(With<BlueEyesWhiteDragonName>, In<MonsterZone>)>,
        >,*/
    );
    type Zone = HandZone;
    fn handle_summon(
        state: &mut MainStep,
        value: <Self::Filter as state_validation::StateFilter<MainStep, Self>>::ValidOutput,
    ) -> (
        ValidPlayerID<()>,
        ValidCardID<(CardIn<Self::Zone>, OfType<MonsterCard>)>,
        crate::identifications::ValidSlotID<In<MonsterZone>>,
    ) {
        // DO NOTHING
        (
            value.player_id.into(),
            value.source_card_id,
            value.slot_id.unchecked_replace_filter(),
        )
    }
}
pub struct BlueEyesWhiteDragonName;
impl StaticName for BlueEyesWhiteDragonName {
    fn name() -> &'static str {
        "Blue-Eyes White Dragon"
    }
}
