use std::{
    any::Any,
    collections::{HashMap, HashSet, hash_map::Entry},
};

use card_stack::priority::{GetState, PriorityMut};
use state_validation::{StateFilter, ValidAction};

use crate::{
    cards::{CardBuilder, CardID},
    events::{
        AddEventListener, AnyClone, DynEventListener, Event, EventActionID, EventActionIDBuilder,
        EventListener, EventListenerConstructor, EventValidAction, SimultaneousActionManager,
    },
    identifications::{ActionID, SourceCardID},
};

#[derive(Debug, Clone)]
pub struct CardManager<EventManager> {
    next_card_id_index: usize,
    event_action_id_builder: EventActionIDBuilder,
    card_actions: CardActions,
    event_manager: EventManager,
    event_tracker: CardEventTracker<EventManager>,
}

impl<EventManager> CardManager<EventManager> {
    pub(crate) fn new(event_manager: EventManager) -> Self {
        CardManager {
            next_card_id_index: 0,
            event_action_id_builder: EventActionIDBuilder::default(),
            card_actions: CardActions {
                card_actions: HashMap::new(),
                card_actions_tracker: HashMap::new(),
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
    pub fn event_tracker(&self) -> &CardEventTracker<EventManager> {
        &self.event_tracker
    }
    pub fn builder(&mut self) -> CardBuilder<'_, EventManager> {
        CardBuilder::new(
            &mut self.card_actions,
            &mut self.event_action_id_builder,
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
    card_actions_tracker: HashMap<CardID, Vec<ActionID>>,
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
                self.card_actions_tracker
                    .get_mut(&card_id)
                    .unwrap()
                    .push(action_id);
            }
            Entry::Vacant(v) => {
                let mut set = HashSet::with_capacity(1);
                let _ = set.insert(card_id);
                let _ = v.insert(set);
                self.card_actions_tracker
                    .insert(card_id, vec![action_id])
                    .unwrap();
            }
        }
    }
    pub fn copy_actions(&mut self, card_id: CardID, to_copy_card_id: CardID) {
        if let Some(actions) = self.card_actions_tracker.get(&to_copy_card_id) {
            for action_id in actions.iter().copied() {
                let _ = self
                    .card_actions
                    .get_mut(&action_id)
                    .unwrap()
                    .insert(card_id);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EventManagerID(usize);
impl EventManagerID {
    pub fn new(index: usize) -> Self {
        EventManagerID(index)
    }
    pub fn index(&self) -> usize {
        self.0
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EventManagerIndex(usize);
impl EventManagerIndex {
    pub(crate) fn new(index: usize) -> Self {
        EventManagerIndex(index)
    }
    pub(crate) fn index(&self) -> usize {
        self.0
    }
}
pub(crate) struct CardEventTracker<EventManager> {
    events: HashMap<
        CardID,
        Vec<(
            EventActionID,
            EventManagerID,
            EventManagerIndex,
            Box<dyn AnyClone>,
            for<'a> fn(
                &'a mut EventManager,
                EventActionID,
                CardID,
                Box<dyn Any>,
            ) -> (EventManagerID, EventManagerIndex),
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
                .map(
                    |(event_action_id, event_manager_id, event_manager_index, input, add_fn)| {
                        (
                            *event_action_id,
                            *event_manager_id,
                            *event_manager_index,
                            input.any_clone_duplication(),
                            *add_fn,
                        )
                    },
                )
                .collect::<Vec<_>>();
            (*card_id, events)
        }) {
            events.insert(card_id, event);
        }
        CardEventTracker { events }
    }
}

impl<EventManager> CardEventTracker<EventManager> {
    pub(crate) fn track_event<
        State,
        Ev: Event<PriorityMut<State>>,
        Listener: EventListenerConstructor<State, Ev>,
    >(
        &mut self,
        card_id: CardID,
        event_action_id: EventActionID,
        id: EventManagerID,
        index: EventManagerIndex,
        listener_input: Listener::Input,
    ) where
        EventManager: AddEventListener<State, Ev>,
        <Listener::Action as EventValidAction<PriorityMut<State>, Listener::ActionInput>>::Output:
            Into<EventManager::Output>,
    {
        let value: (
            EventActionID,
            EventManagerID,
            EventManagerIndex,
            Box<dyn AnyClone>,
            for<'a> fn(
                &'a mut EventManager,
                EventActionID,
                CardID,
                Box<dyn Any>,
            ) -> (EventManagerID, EventManagerIndex),
        ) = (
            event_action_id,
            id,
            index,
            Box::new(listener_input),
            |event_manager: &mut EventManager,
             event_action_id: EventActionID,
             card_id: CardID,
             listener_input: Box<dyn Any>| {
                let listener = Listener::new_listener(
                    SourceCardID(card_id),
                    *listener_input.downcast::<Listener::Input>().unwrap(),
                );
                event_manager.add_listener(event_action_id, listener)
            },
        );
        match self.events.entry(card_id) {
            Entry::Vacant(v) => {
                v.insert(vec![value]);
            }
            Entry::Occupied(o) => o.into_mut().push(value),
        }
    }
    pub(crate) fn copy_events(
        &mut self,
        event_manager: &mut EventManager,
        card_id: CardID,
        to_copy_card_id: CardID,
    ) {
        if let Some(events) = self.events.get(&to_copy_card_id) {
            for (event_action_id, _, _, listener_input, add_event) in events {
                add_event(
                    event_manager,
                    *event_action_id,
                    card_id,
                    (**listener_input).any_clone_duplication(),
                );
            }
        }
    }
    pub(crate) fn events_for_card<'a>(
        &'a self,
        card_id: CardID,
    ) -> Option<impl Iterator<Item = (EventManagerID, EventManagerIndex)> + 'a> {
        if let Some(events) = self.events.get(&card_id) {
            Some(events.iter().map(|(_, id, index, ..)| (*id, *index)))
        } else {
            None
        }
    }
}
