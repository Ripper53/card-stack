use card_game::stack::NonEmptyInput;
use state_validation::StateFilterInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterID(usize);
impl NonEmptyInput for CharacterID {}
impl CharacterID {
    pub fn new(id: usize) -> Self {
        CharacterID(id)
    }
}
