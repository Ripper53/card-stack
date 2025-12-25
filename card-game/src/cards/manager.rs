use std::{
    any::Any,
    collections::{HashMap, HashSet, hash_map::Entry},
};

use card_stack::priority::{GetState, PriorityMut};
use state_validation::ValidAction;

use crate::{
    cards::{CardBuilder, CardID},
    events::{AddEventListener, AnyClone, DynEventListener, Event, EventListener},
    identifications::ActionID,
};

#[derive(Debug, Clone)]
pub struct CardManager<EventManager> {
    next_card_id_index: usize,
    card_actions: CardActions,
    event_manager: EventManager,
    event_tracker: CardEventTracker<EventManager>,
}

impl<EventManager> CardManager<EventManager> {
    pub(crate) fn new(event_manager: EventManager) -> Self {
        CardManager {
            next_card_id_index: 0,
            card_actions: CardActions {
                card_actions: HashMap::new(),
            },
            event_manager,
            event_tracker: CardEventTracker {
                events: HashMap::new(),
            },
        }
    }
    pub fn card_actions(&self) -> &CardActions {
        &self.card_actions
    }
    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }
    pub fn builder(&mut self) -> CardBuilder<'_, EventManager> {
        CardBuilder::new(
            &mut self.card_actions,
            &mut self.event_manager,
            &mut self.event_tracker,
            &mut self.next_card_id_index,
        )
    }
}
impl<EventManager> GetState<CardActions> for CardManager<EventManager> {
    fn state(&self) -> &CardActions {
        self.card_actions()
    }
}

#[derive(Debug, Clone)]
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

pub(crate) struct CardEventTracker<EventManager> {
    events: HashMap<
        CardID,
        Vec<(
            Box<dyn AnyClone>,
            for<'a> fn(&'a mut EventManager, Box<dyn Any>),
        )>,
    >,
}
impl<EventManager> std::fmt::Debug for CardEventTracker<EventManager> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardEventTracker").finish_non_exhaustive()
    }
}
impl<EventManager> Clone for CardEventTracker<EventManager> {
    fn clone(&self) -> Self {
        let mut events = HashMap::with_capacity(self.events.len());
        for (card_id, event) in self.events.iter().map(|(card_id, events)| {
            let events = events
                .iter()
                .map(|(input, add_fn)| (input.any_clone(), *add_fn))
                .collect::<Vec<_>>();
            (*card_id, events)
        }) {
            events.insert(card_id, event);
        }
        CardEventTracker { events }
    }
}

impl<EventManager> CardEventTracker<EventManager> {
    pub fn track_event<State, Ev: Event<PriorityMut<State>>, Listener: EventListener<State, Ev>>(
        &mut self,
        card_id: CardID,
        listener: Listener,
    ) where
        EventManager: AddEventListener<State, Ev>,
        <Listener::Action as ValidAction<PriorityMut<State>, Listener::ActionInput>>::Output:
            Into<EventManager::Output>,
    {
        let value: (
            Box<dyn AnyClone>,
            for<'a> fn(&'a mut EventManager, Box<dyn Any>),
        ) = (
            Box::new(listener),
            |event_manager: &mut EventManager, listener: Box<dyn Any>| {
                event_manager.add_listener(*listener.downcast::<Listener>().unwrap())
            },
        );
        match self.events.entry(card_id) {
            Entry::Vacant(v) => {
                v.insert(vec![value]);
            }
            Entry::Occupied(o) => o.into_mut().push(value),
        }
    }
    pub fn copy_events(
        &mut self,
        event_manager: &mut EventManager,
        card_id: CardID,
        to_copy_card_id: CardID,
    ) {
        if let Some(events) = self.events.get(&to_copy_card_id) {
            for (listener, add_event) in events {
                add_event(event_manager, listener.any_clone());
            }
        }
    }
}
