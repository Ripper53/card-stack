use card_game::stack::{
    priority::{GetState, Priority, PriorityError, PriorityStack},
    requirements::{ActionRequirement, RequirementAction},
};
use state_validation::CollectedInputs;

use crate::{
    filters::Using,
    game::{Game, GetStateMut},
    identifications::CharacterID,
};

pub struct TargetCharacter;

impl<State: GetState<Game>> ActionRequirement<State, CharacterID> for TargetCharacter {
    type Filter = Using<CharacterID>;
    fn collect_inputs(state: &State) -> CollectedInputs<State, impl Iterator<Item = CharacterID>> {
        CollectedInputs::new(state.state().characters.keys().copied())
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
