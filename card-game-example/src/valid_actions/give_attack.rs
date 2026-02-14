use card_game::{
    ActionInfo,
    cards::{CardCommandManager, CardID},
    events::{AddEventListener, EventListener, EventListenerConstructor},
    identifications::{
        ActionID, ActionIdentifier, MutID, PlayerID, SourceCardID, ValidCardID, ValidPlayerID,
    },
    stack::priority::GetState,
    zones::Zone,
};
use state_validation::{Condition, StateFilter, StateFilterConversion, ValidAction};

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
impl<F>
    ActionInfo<
        (),
        (
            ValidPlayerID<F>,
            MutID<ValidCardID<(CardIn<MonsterZone>, OfType<MonsterCard>)>>,
        ),
    > for GiveAttack
{
    fn info(
        &self,
        input: &(
            ValidPlayerID<F>,
            MutID<ValidCardID<(CardIn<MonsterZone>, OfType<MonsterCard>)>>,
        ),
    ) {
    }
}
impl<State: GetStateMut<Game>> ValidAction<State, (PlayerID, CardID)> for GiveAttack {
    type Filter = (
        Condition<(PlayerID, CardID), CardIn<MonsterZone>>,
        Condition<
            (
                ValidPlayerID<()>,
                ValidCardID<(CardIn<MonsterZone>, OfType<MonsterZoneCard>)>,
            ),
            OfType<MonsterCard>,
        >,
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
        let r = CardCommandManager::new(valid_card_id)
            // TODO
            .with_history(card_game::ActionHistory::new())
            .validate_with_input(state, move |valid_card_id| (valid_player_id, valid_card_id))
            .unwrap();
        r.execute(self)
    }
}
impl<State: GetStateMut<Game>, F>
    ValidAction<
        State,
        (
            ValidPlayerID<F>,
            MutID<ValidCardID<<MonsterZone as Zone>::CardFilter>>,
        ),
    > for GiveAttack
{
    type Filter = OfType<MonsterCard>;
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter<
            State,
            (
                ValidPlayerID<F>,
                MutID<ValidCardID<<MonsterZone as Zone>::CardFilter>>,
            ),
        >>::ValidOutput,
    ) -> Self::Output {
        let card = state
            .state_mut()
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_monster_card_mut(valid_card_id);
        card.add_attack(self.attack);
        state
    }
}
impl<State: GetStateMut<Game>, F>
    ValidAction<
        State,
        (
            ValidPlayerID<F>,
            MutID<ValidCardID<(CardIn<MonsterZone>, OfType<MonsterCard>)>>,
        ),
    > for GiveAttack
{
    type Filter = ();
    type Output = State;
    fn with_valid_input(
        self,
        mut state: State,
        (valid_player_id, valid_card_id): <Self::Filter as StateFilter<
            State,
            (
                ValidPlayerID<F>,
                MutID<ValidCardID<(CardIn<MonsterZone>, OfType<MonsterCard>)>>,
            ),
        >>::ValidOutput,
    ) -> Self::Output {
        let card = state
            .state_mut()
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_monster_card_mut(valid_card_id);
        card.add_attack(self.attack);
        state
    }
}
impl<State: GetStateMut<Game>> ValidAction<State, Summoned> for GiveAttack {
    type Filter = (
        Condition<(PlayerID, CardID), CardIn<MonsterZone>>,
        Condition<
            (
                ValidPlayerID<()>,
                ValidCardID<<MonsterZone as Zone>::CardFilter>,
            ),
            OfType<MonsterCard>,
        >,
    );
    type Output = <Self as ValidAction<State, (PlayerID, CardID)>>::Output;
    fn with_valid_input(
        self,
        state: State,
        valid: <Self::Filter as StateFilter<State, Summoned>>::ValidOutput,
    ) -> Self::Output {
        <Self as ValidAction<State, (PlayerID, CardID)>>::with_valid_input(self, state, valid)
    }
}

#[derive(StateFilterConversion, Clone)]
pub struct PassiveGiveAttack {
    #[conversion(T = ValidCardID<T>)]
    #[conversion(T = (ValidPlayerID<()>, ValidCardID<T>))]
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
    type Filter = Condition<CardID, CardIn<MonsterZone>>;
    type Action = GiveAttack;
    fn action(
        &self,
        _state: &State,
        _event: &Summoned,
        _value: <Self::Filter as StateFilter<State, Self>>::ValidOutput,
    ) -> Self::Action {
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
