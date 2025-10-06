use card_game::{
    cards::{ActionID, CardID},
    events::{EventListener, EventListenerConstructor, GetEventManagerMut},
    identifications::{PlayerID, SourceCardID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{Condition, StateFilter, ValidAction},
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::{Attack, MonsterCard, MonsterCardType, MonsterZoneCard},
    events::{EventManager, summon::SpecialSummoned},
    filters::{CardIn, FilterInput, IntoInput, OfType},
    steps::GetStateMut,
    zones::monster::MonsterZone,
};

#[derive(Clone, Debug)]
pub struct GiveAttack {
    attack: Attack,
}

impl GiveAttack {
    pub fn new(attack: Attack) -> Self {
        GiveAttack { attack }
    }
}

impl<State: GetStateMut<Game>> ValidAction<State, FilterInput<(PlayerID, CardID)>> for GiveAttack {
    type Filter = (
        Condition<FilterInput<(PlayerID, CardID)>, CardIn<MonsterZone>>,
        Condition<
            FilterInput<(ValidPlayerID<()>, ValidCardID<CardIn<MonsterZone>>)>,
            OfType<MonsterCard>,
        >,
    );
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        FilterInput((valid_player_id, valid_card_id)): <Self::Filter as card_game::validation::StateFilter<
            State,
            FilterInput<(PlayerID, CardID)>,
        >>::ValidOutput,
    ) -> Self::Output {
        let card = state
            .state_mut()
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_card_mut(valid_card_id.into());
        card.kind_mut().kind_mut().add_attack(self.attack);
        state
    }
    fn action_id() -> ActionID {
        ActionID::new("give_attack")
    }
}

impl<State: GetStateMut<Game>, F>
    ValidAction<State, FilterInput<(ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>)>>
    for GiveAttack
{
    type Filter = OfType<MonsterCard>;
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        FilterInput((valid_player_id, valid_card_id)): <Self::Filter as StateFilter<
            State,
            FilterInput<(ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>)>,
        >>::ValidOutput,
    ) -> Self::Output {
        let card = state
            .state_mut()
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_card_mut(valid_card_id.into());
        card.kind_mut().kind_mut().add_attack(self.attack);
        state
    }
    fn action_id() -> ActionID {
        ActionID::new("give_attack")
    }
}

pub struct PassiveGiveAttack {
    source_card_id: CardID,
    give_attack: GiveAttack,
}

impl PassiveGiveAttack {
    pub fn new(source_card_id: CardID, give_attack: GiveAttack) -> Self {
        PassiveGiveAttack {
            source_card_id,
            give_attack,
        }
    }
}
impl<State: GetStateMut<Game>> EventListener<State, SpecialSummoned> for PassiveGiveAttack {
    type Filter = CardIn<MonsterZone>;
    type Action = GiveAttack;
    fn action(&self, _event: &SpecialSummoned) -> Self::Action {
        self.give_attack.clone()
    }
}
impl<State: GetStateMut<Game>> EventListenerConstructor<State, SpecialSummoned>
    for PassiveGiveAttack
{
    type Input = GiveAttack;
    fn new_listener(source_card_id: SourceCardID, give_attack: Self::Input) -> Self {
        PassiveGiveAttack {
            source_card_id: source_card_id.0,
            give_attack,
        }
    }
}
