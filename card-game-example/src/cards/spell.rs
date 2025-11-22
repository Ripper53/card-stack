use crate::cards::CardName;

#[derive(Debug)]
pub struct SpellCard {}

impl CardName for SpellCard {
    fn name(&self) -> &super::Name {
        todo!()
    }
}
