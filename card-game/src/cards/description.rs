use std::collections::{HashMap, HashSet, hash_map::Entry};

use crate::{cards::CardID, events::EventActionID};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct CardDescriptions<T> {
    descriptions: HashMap<CardID, Vec<T>>,
    event_ids: HashSet<EventActionID>,
}

impl<T> CardDescriptions<T> {
    pub fn new() -> Self {
        CardDescriptions {
            descriptions: HashMap::new(),
            event_ids: HashSet::new(),
        }
    }
    pub(crate) fn add_description(&mut self, card_id: CardID, description: T) {
        match self.descriptions.entry(card_id) {
            Entry::Occupied(o) => {
                o.into_mut().push(description);
            }
            Entry::Vacant(v) => {
                let _ = v.insert(vec![description]);
            }
        }
    }
    pub(crate) fn add_event_description(
        &mut self,
        event_id: EventActionID,
        card_id: CardID,
        description: T,
    ) {
        if self.event_ids.insert(event_id) {
            self.add_description(card_id, description);
        }
    }
    pub fn descriptions(&self, card_id: CardID) -> Option<&Vec<T>> {
        self.descriptions.get(&card_id)
    }
}

impl<T: Clone> CardDescriptions<T> {
    pub(crate) fn copy_description(&mut self, card_id: CardID, copy_from_card_id: CardID) {
        let Some(descriptions) = self.descriptions.get(&card_id).cloned() else {
            return;
        };
        match self.descriptions.entry(card_id) {
            Entry::Occupied(o) => {
                o.into_mut().extend(descriptions);
            }
            Entry::Vacant(v) => {
                let _ = v.insert(descriptions);
            }
        }
    }
}
