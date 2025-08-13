use card_game::stack::{
    priority::{GetState, Priority, PriorityError, PriorityStack},
    requirements::{ActionRequirement, RequirementAction, SatisfyRequirement},
};

use crate::{game::TurnState, identifications::CharacterID};

pub struct TargetCharacter;

impl<State: TurnState> SatisfyRequirement<Priority<State>> for TargetCharacter {
    type Value = CharacterID;
    type RequirementError = InvalidTargetCharacterError;
    fn satisfy(
        &self,
        _priority: &Priority<State>,
        _value: &Self::Value,
    ) -> Result<(), Self::RequirementError> {
        Ok(())
    }
    fn single_selection(&self, priority: &Priority<State>) -> Option<Self::Value> {
        let game = priority.state().game();
        if game.characters.len() == 1 {
            let target = game.characters.keys().copied().next().unwrap();
            Some(target)
        } else {
            None
        }
    }
    fn force_selection(&self, priority: &Priority<State>) -> Self::Value {
        priority
            .state()
            .game()
            .characters
            .keys()
            .copied()
            .next()
            .unwrap()
    }
}
impl<State: TurnState, IncitingAction: card_game::stack::actions::IncitingAction<State>>
    SatisfyRequirement<PriorityStack<State, IncitingAction>> for TargetCharacter
{
    type Value = CharacterID;
    type RequirementError = InvalidTargetCharacterError;
    fn satisfy(
        &self,
        _priority: &PriorityStack<State, IncitingAction>,
        _value: &Self::Value,
    ) -> Result<(), Self::RequirementError> {
        Ok(())
    }
    fn single_selection(
        &self,
        priority: &PriorityStack<State, IncitingAction>,
    ) -> Option<Self::Value> {
        let game = priority.state().game();
        if game.characters.len() == 1 {
            let target = game.characters.keys().copied().next().unwrap();
            Some(target)
        } else {
            None
        }
    }
    fn force_selection(&self, priority: &PriorityStack<State, IncitingAction>) -> Self::Value {
        priority
            .state()
            .game()
            .characters
            .keys()
            .copied()
            .next()
            .unwrap()
    }
}
impl<State: TurnState, Action: card_game::stack::actions::ActionSource>
    ActionRequirement<Priority<State>, Action> for TargetCharacter
{
    type Satisfy = TargetCharacter;
    type RequirementError = NoTargetCharacterError;
    fn can_satisfy(
        &self,
        priority: Priority<State>,
        action: Action,
        source: Action::Source,
    ) -> Result<
        RequirementAction<Priority<State>, Action, Self::Satisfy>,
        PriorityError<Priority<State>, Self::RequirementError>,
    > {
        let game = priority.state().game();
        if game.characters.is_empty() {
            Err(PriorityError {
                priority,
                error: NoTargetCharacterError,
            })
        } else {
            Ok(RequirementAction::new(
                priority,
                action,
                source,
                TargetCharacter,
            ))
        }
    }
}
impl<
    State: TurnState,
    Action: card_game::stack::actions::ActionSource,
    IncitingAction: card_game::stack::actions::IncitingAction<State>,
> ActionRequirement<PriorityStack<State, IncitingAction>, Action> for TargetCharacter
{
    type Satisfy = TargetCharacter;
    type RequirementError = NoTargetCharacterError;
    fn can_satisfy(
        &self,
        priority: PriorityStack<State, IncitingAction>,
        action: Action,
        source: Action::Source,
    ) -> Result<
        RequirementAction<PriorityStack<State, IncitingAction>, Action, Self::Satisfy>,
        PriorityError<PriorityStack<State, IncitingAction>, Self::RequirementError>,
    > {
        let game = priority.state().game();
        if game.characters.is_empty() {
            Err(PriorityError {
                priority,
                error: NoTargetCharacterError,
            })
        } else {
            Ok(RequirementAction::new(
                priority,
                action,
                source,
                TargetCharacter,
            ))
        }
    }
}
#[derive(thiserror::Error, Debug)]
#[error("no available character target")]
pub struct NoTargetCharacterError;

#[derive(Debug)]
pub struct InvalidTargetCharacterError(CharacterID);
impl std::error::Error for InvalidTargetCharacterError {}
impl std::fmt::Display for InvalidTargetCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid target character {:?}", self.0)
    }
}
