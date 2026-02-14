use card_game::{
    identifications::ValidCardID,
    stack::{
        actions::IncitingAction,
        priority::{Priority, PriorityMut}, requirements::{ActionRequirement, SatisfyRequirement},
    },
    zones::Zone,
};

use crate::{
    Game, cards::monster::Attack, filters::CardIn, steps::GetStateMut, zones::monster::MonsterZone,
};

pub struct GiveAttack {
    valid_card_id: ValidCardID<CardIn<MonsterZone>>,
    attack: Attack,
}

impl<State: GetStateMut<Game>> SatisfyRequirement<Priority<State>> for GiveAttack {
    type Value = ValidCardID<CardIn<MonsterZone>>;
    type RequirementError = std::convert::Infallible;
    fn satisfy(
            &self,
            _priority: &Priority<State>,
            _value: &Self::Value,
        ) -> Result<(), Self::RequirementError> {
        Ok(())
    }
    fn single_selection(&self, priority: &Priority<State>) -> Option<Self::Value> {
        todo!()
    }
    fn force_selection(&self, priority: &Priority<State>) -> Self::Value {
        todo!()
    }
}
impl ActionRequirement<Priority<State>, Self> for GiveAttack {
    type Satisfy = ;
    type RequirementError = ;
    fn can_satisfy(
            &self,
            priority: Priority<State>,
            action: Self,
            source: <Self as card_game::stack::actions::ActionSource>::Source,
        ) -> Result<
            card_game::stack::requirements::RequirementAction<Priority<State>, Self, Self::Satisfy>,
            card_game::stack::priority::PriorityError<Priority<State>, Self::RequirementError>,
        > {
        
    }
}
impl<State: GetStateMut<Game>> IncitingAction<State> for GiveAttack {
    type EmptyStackRequirement = ();
    fn requirement(&self) -> Self::EmptyStackRequirement {}

    type Stackable = StackAction;
    type ResolvedIncitingAction = Priority<State>;
    fn resolve(self, priority: PriorityMut<Priority<State>>) -> Self::ResolvedIncitingAction {
        let state = priority.state_mut().state_mut();
        let card = state
            .zone_manager_mut()
            .valid_zone_mut(valid_player_id)
            .monster_zone
            .valid_card_mut(valid_card_id.into());
        card.kind_mut().kind_mut().add_attack(self.attack);
        priority.take_priority()
    }
}
