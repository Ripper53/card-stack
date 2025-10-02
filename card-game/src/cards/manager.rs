use std::collections::{HashMap, HashSet, hash_map::Entry};

use crate::{
    cards::{CardBuilder, CardID},
    events::{Event, EventListener},
    validation::ValidAction,
};

pub struct CardManager {
    next_card_id_index: usize,
    card_actions: CardActions,
}

impl CardManager {
    pub(crate) fn new() -> Self {
        CardManager {
            next_card_id_index: 0,
            card_actions: CardActions {
                card_actions: HashMap::new(),
            },
        }
    }
    pub(crate) fn card_actions(&self) -> &CardActions {
        &self.card_actions
    }
    pub fn builder(&mut self) -> CardBuilder<'_> {
        CardBuilder::new(&mut self.card_actions, &mut self.next_card_id_index)
    }
}

pub(crate) struct CardActions {
    card_actions: HashMap<ActionID, HashSet<CardID>>,
}
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct ActionID(&'static str);
impl ActionID {
    pub fn new(value: &'static str) -> Self {
        ActionID(value)
    }
}

impl CardActions {
    pub fn contains_action(&self, action_id: ActionID, card_id: CardID) -> bool {
        if let Some(cards) = self.card_actions.get(&action_id) {
            cards.contains(&card_id)
        } else {
            false
        }
    }
    pub fn insert_action(&mut self, action_id: ActionID, card_id: CardID) {
        match self.card_actions.entry(action_id) {
            Entry::Occupied(o) => {
                let _ = o.into_mut().insert(card_id);
            }
            Entry::Vacant(v) => {
                let mut set = HashSet::with_capacity(1);
                let _ = set.insert(card_id);
                let _ = v.insert(set);
            }
        }
    }
}
