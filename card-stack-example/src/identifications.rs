use state_validation::StateFilterInput;

#[derive(StateFilterInput, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacterID(usize);
impl CharacterID {
    pub fn new(id: usize) -> Self {
        CharacterID(id)
    }
}
