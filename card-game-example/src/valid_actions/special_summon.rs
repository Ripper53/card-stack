use card_game::{
    cards::{Card, CardID},
    identifications::{ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{StateFilter, StateFilterInput, ValidAction},
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::{MonsterCard, MonsterZoneCard, Position},
    filters::{CardIn, FilterInput, In, OfType},
    identifications::ValidSlotID,
    steps::GetStateMut,
    zones::{ContainsMonsterCards, GetZone, hand::HandZone, monster::MonsterZone},
};

pub struct SpecialSummonValidAction {
    position: Position,
}

impl SpecialSummonValidAction {
    pub fn new(position: Position) -> Self {
        SpecialSummonValidAction { position }
    }
}

impl<State: GetStateMut<Game>, Requirement: SpecialSummonRequirement<State>>
    ValidAction<State, Requirement> for SpecialSummonValidAction
{
    type Filter = Requirement::Filter;
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        valid: <Self::Filter as card_game::validation::StateFilter<
            State,
            Requirement,
        >>::ValidOutput,
    ) -> Self::Output {
        let (valid_player_id, valid_card_id, valid_slot_id) = Requirement::handle_summon(valid);
        let zone = <Requirement::Zone as ContainsMonsterCards>::get_zone_mut(
            state.state_mut(),
            valid_player_id.unchecked_clone(),
        );
        let card = zone.remove_monster_card(valid_card_id.into());
        let card_id = card.id();
        let card = MonsterZoneCard::new(card.take_kind().into(), self.position);
        let _ = state
            .state_mut()
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_slot(valid_slot_id)
            .put(Card::new(card_id, card).into_kind());
        state
    }
}

pub trait SpecialSummonRequirement<State>: StateFilterInput + Sized {
    type Filter: StateFilter<State, Self>;
    type Zone: ContainsMonsterCards;
    fn handle_summon(
        input: <Self::Filter as StateFilter<State, Self>>::ValidOutput,
    ) -> (
        ValidPlayerID<()>,
        ValidCardID<(CardIn<Self::Zone>, OfType<MonsterCard>)>,
        ValidSlotID<In<MonsterZone>>,
    );
}
