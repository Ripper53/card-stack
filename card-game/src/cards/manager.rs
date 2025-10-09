use std::collections::{HashMap, HashSet, hash_map::Entry};

use card_stack::priority::GetState;
use card_validation::ActionID;

use crate::{
    cards::{CardBuilder, CardID},
    events::{Event, EventListener},
    validation::ValidAction,
};

pub struct CardManager<EventManager> {
    next_card_id_index: usize,
    card_actions: CardActions,
    event_manager: EventManager,
}

impl<EventManager> CardManager<EventManager> {
    pub(crate) fn new(event_manager: EventManager) -> Self {
        CardManager {
            next_card_id_index: 0,
            card_actions: CardActions {
                card_actions: HashMap::new(),
            },
            event_manager,
        }
    }
    pub fn card_actions(&self) -> &CardActions {
        &self.card_actions
    }
    pub fn builder(&mut self) -> CardBuilder<'_, EventManager> {
        CardBuilder::new(
            &mut self.card_actions,
            &mut self.event_manager,
            &mut self.next_card_id_index,
        )
    }
}
impl<EventManager> GetState<CardActions> for CardManager<EventManager> {
    fn state(&self) -> &CardActions {
        self.card_actions()
    }
}

pub struct CardActions {
    card_actions: HashMap<ActionID, HashSet<CardID>>,
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
