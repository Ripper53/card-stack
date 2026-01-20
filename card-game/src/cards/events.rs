use card_stack::priority::{Priority, PriorityMut};

use crate::{
    cards::{CardID, CardManager},
    events::{CollectedActions, Event, EventAction, EventManager, SimultaneousActionManager},
};

#[derive(Debug)]
pub struct EventCommand<State> {
    state: State,
}

impl<State> EventCommand<State> {
    pub fn new(state: State) -> Self {
        EventCommand { state }
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn take_state(self) -> State {
        self.state
    }
}

pub trait EventForCardExists<State, Ev: Event<PriorityMut<State>>, Output: 'static, EvM> {
    fn event_for_card_exists(
        &self,
        card_id: CardID,
        event: &Ev,
        get_card_manager: for<'a> fn(&'a State) -> &'a CardManager<EvM>,
        get_event_manager: for<'a> fn(&'a EvM) -> &'a EventManager<State, Ev, Output>,
    ) -> bool;
}
pub trait EventsForCard<State, Ev: Event<PriorityMut<State>>, Output: 'static, EvM> {
    fn events_for_card(
        self,
        card_id: CardID,
        event: Ev,
        get_card_manager: for<'a> fn(&'a State) -> &'a CardManager<EvM>,
        get_event_manager: for<'a> fn(&'a EvM) -> &'a EventManager<State, Ev, Output>,
    ) -> Result<SimultaneousActionManager<State, Ev, Output>, NoEventsFound<State>>;
}

impl<'a, State, Ev: Event<PriorityMut<Priority<State>>>, Output: 'static, EvM>
    EventForCardExists<Priority<State>, Ev, Output, EvM> for EventCommand<&'a Priority<State>>
{
    fn event_for_card_exists(
        &self,
        card_id: CardID,
        event: &Ev,
        get_card_manager: for<'b> fn(&'b Priority<State>) -> &'b CardManager<EvM>,
        get_event_manager: for<'b> fn(&'b EvM) -> &'b EventManager<Priority<State>, Ev, Output>,
    ) -> bool {
        let card_manager = get_card_manager(&self.state);
        let event_tracker = card_manager.event_tracker();
        if let Some(mut event_indexes) = event_tracker.events_for_card(card_id) {
            let event_manager = get_event_manager(card_manager.event_manager());
            event_indexes.any(|index| {
                if let Some((_id, ev)) = event_manager.events().get(index.1.index()) {
                    ev.get_action(self.state, event).is_ok()
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}
impl<State: 'static, Ev: Event<PriorityMut<Priority<State>>>, Output: 'static, EvM>
    EventsForCard<Priority<State>, Ev, Output, EvM> for EventCommand<Priority<State>>
{
    fn events_for_card(
        self,
        card_id: CardID,
        event: Ev,
        get_card_manager: for<'a> fn(&'a Priority<State>) -> &'a CardManager<EvM>,
        get_event_manager: for<'a> fn(&'a EvM) -> &'a EventManager<Priority<State>, Ev, Output>,
    ) -> Result<
        SimultaneousActionManager<Priority<State>, Ev, Output>,
        NoEventsFound<Priority<State>>,
    > {
        let card_manager = get_card_manager(&self.state);
        let event_tracker = card_manager.event_tracker();
        let Some(event_indexes) = event_tracker.events_for_card(card_id) else {
            return Err(NoEventsFound { state: self.state });
        };
        let event_manager = get_event_manager(card_manager.event_manager());
        let (indexes, events) = event_indexes
            .filter_map(|(event_manager_id, index)| {
                if let Some((event_action_id, ev)) = event_manager.events().get(index.index())
                    && let Ok(event_action) = ev.get_action(&self.state, &event)
                {
                    Some(((event_manager_id, index), (*event_action_id, event_action)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip();
        Ok(
            CollectedActions::<Priority<State>, _, _>::new(event, indexes, events)
                .simultaneous_action_manager(self.state),
        )
    }
}
impl<State: 'static, Ev: Event<PriorityMut<Priority<State>>>, Output: 'static>
    EventCommand<SimultaneousActionManager<Priority<State>, Ev, Output>>
{
    pub fn event_triggered_for_card<EvM>(
        &self,
        card_id: CardID,
        get_card_manager: for<'a> fn(
            &'a SimultaneousActionManager<Priority<State>, Ev, Output>,
        ) -> &'a CardManager<EvM>,
    ) -> bool {
        let card_manager = get_card_manager(&self.state);
        let event_tracker = card_manager.event_tracker();
        if let Some(mut event_indexes) = event_tracker.events_for_card(card_id) {
            event_indexes.any(|index| self.state.indexes.contains(&index))
        } else {
            false
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("no events found")]
pub struct NoEventsFound<State> {
    state: State,
}

impl<State> NoEventsFound<State> {
    pub fn take_state(self) -> State {
        self.state
    }
}
