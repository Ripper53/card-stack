use card_game::{
    cards::{Card, CardID},
    events::TriggeredEvent,
    identifications::{ActionID, ActionIdentifier, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{StateFilter, StateFilterInput, ValidAction},
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::{MonsterCard, MonsterZoneCard, Position},
    events::summon::SpecialSummoned,
    filters::{CardIn, FilterInput, In, OfType},
    identifications::ValidSlotID,
    steps::GetStateMut,
    zones::{ContainsMonsterCards, GetZone, hand::HandZone, monster::MonsterZone},
};

pub struct SpecialSummon<State, Requirement: SpecialSummonRequirement<State>> {
    position: Position,
    _m: std::marker::PhantomData<(State, Requirement)>,
}

impl<State, Requirement: SpecialSummonRequirement<State>> SpecialSummon<State, Requirement> {
    pub fn new(position: Position) -> Self {
        SpecialSummon {
            position,
            _m: std::marker::PhantomData::default(),
        }
    }
}

impl<State: GetStateMut<Game>, Requirement: SpecialSummonRequirement<State>>
    ValidAction<State, Requirement> for SpecialSummon<State, Requirement>
{
    type Filter = Requirement::Filter;
    type Output = TriggeredEvent<State, SpecialSummoned>;
    fn with_valid_input(
        self,
        mut state: State,
        valid: <Self::Filter as card_game::validation::StateFilter<
            State,
            Requirement,
        >>::ValidOutput,
    ) -> Self::Output {
        let (valid_player_id, valid_card_id, valid_slot_id) =
            Requirement::handle_summon(&mut state, valid);
        let zone = <Requirement::Zone as ContainsMonsterCards>::get_zone_mut(
            state.state_mut(),
            valid_player_id.unchecked_clone(),
        );
        let card = zone.remove_monster_card(valid_card_id.into());
        let card_id = card.id();
        let card = MonsterZoneCard::new(card.take_kind().into(), self.position);
        let player_id = valid_player_id.id();
        let _ = state
            .state_mut()
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_slot(valid_slot_id)
            .put(Card::new(card_id, card).into_kind());
        TriggeredEvent::new(
            state,
            SpecialSummoned(card_id),
            FilterInput((player_id, card_id)),
        )
    }
}
impl<State, Requirement: SpecialSummonRequirement<State>> ActionIdentifier
    for SpecialSummon<State, Requirement>
{
    fn action_id() -> ActionID {
        Requirement::action_id()
    }
}

pub trait SpecialSummonRequirement<State>: ActionIdentifier + StateFilterInput + Sized {
    type Filter: StateFilter<State, Self>;
    type Zone: ContainsMonsterCards;
    fn handle_summon(
        state: &mut State,
        input: <Self::Filter as StateFilter<State, Self>>::ValidOutput,
    ) -> (
        ValidPlayerID<()>,
        ValidCardID<(CardIn<Self::Zone>, OfType<MonsterCard>)>,
        ValidSlotID<In<MonsterZone>>,
    );
}
