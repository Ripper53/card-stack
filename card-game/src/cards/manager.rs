use crate::cards::CardBuilder;

pub struct CardManager {
    card_builder: CardBuilder,
}

impl CardManager {
    pub(crate) fn new(card_builder: CardBuilder) -> Self {
        CardManager { card_builder }
    }
    pub fn builder(&mut self) -> &mut CardBuilder {
        &mut self.card_builder
    }
}
