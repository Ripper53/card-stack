use card_game::{
    cards::CardID,
    events::{AddEventListener, EventListener, EventListenerConstructor},
    identifications::{
        ActionID, ActionIdentifier, PlayerID, SourceCardID, ValidCardID, ValidPlayerID,
    },
    stack::priority::GetState,
    zones::Zone,
};
use state_validation::{Condition, StateFilter, ValidAction};

use crate::{
    Game,
    cards::monster::{Attack, MonsterCard, MonsterCardType, MonsterZoneCard},
    events::{
        EventManager,
        summon::{SpecialSummoned, Summoned},
    },
    filters::{CardIn, OfType},
    steps::GetStateMut,
    zones::{hand::HandZone, monster::MonsterZone},
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

impl ActionIdentifier for GiveAttack {
    fn action_id() -> ActionID {
        ActionID::new("give_attack")
    }
}
impl<State: GetStateMut<Game>> ValidAction<State, (PlayerID, CardID)> for GiveAttack {
    type Filter = (
        Condition<(PlayerID, CardID), CardIn<MonsterZone>>,
        Condition<(ValidPlayerID<()>, ValidCardID<CardIn<MonsterZone>>), OfType<MonsterCard>>,
    );
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        (valid_player_id, valid_card_id): <Self::Filter as state_validation::StateFilter<
            State,
            (PlayerID, CardID),
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
}

impl<State: GetStateMut<Game>, F>
    ValidAction<State, (ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>)> for GiveAttack
{
    type Filter = OfType<MonsterCard>;
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter<
            State,
            (ValidPlayerID<F>, ValidCardID<CardIn<MonsterZone>>),
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
}

#[derive(Clone)]
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
impl<State: GetStateMut<Game>> EventListener<State, Summoned> for PassiveGiveAttack {
    type Filter = CardIn<MonsterZone>;
    type Action = GiveAttack;
    fn action(&self, _state: &State, _event: &Summoned) -> Self::Action {
        self.give_attack.clone()
    }
}
impl<State: GetStateMut<Game>> EventListenerConstructor<State, Summoned> for PassiveGiveAttack {
    type Input = GiveAttack;
    fn new_listener(source_card_id: SourceCardID, give_attack: Self::Input) -> Self {
        PassiveGiveAttack {
            source_card_id: source_card_id.0,
            give_attack,
        }
    }
}
