use card_game::{
    cards::{ActionID, CardID},
    identifications::{PlayerID, ValidCardID, ValidPlayerID},
    stack::priority::GetState,
    validation::{Condition, ValidAction},
    zones::Zone,
};

use crate::{
    Game,
    cards::monster::{Attack, MonsterCard, MonsterCardType, MonsterZoneCard},
    filters::{CardIn, FilterInput, OfType},
    steps::GetStateMut,
    zones::monster::MonsterZone,
};

pub struct GiveAttack {
    attack: Attack,
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
