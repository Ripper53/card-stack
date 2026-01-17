use card_stack::priority::{Priority, PriorityMut};

use crate::{
    cards::{CardEventTracker, CardID, CardManager},
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
        let event_manager = get_event_manager(card_manager.event_manager());
        if let Some(mut event_indexes) = event_tracker.events_for_card(card_id) {
            event_indexes.any(|index| {
                if let Some(ev) = event_manager.events().get(index.value()) {
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
        let events = event_indexes
            .filter_map(|index| {
                if let Some(ev) = event_manager.events().get(index.value()) {
                    ev.get_action(&self.state, &event).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Ok(
            CollectedActions::<Priority<State>, _, _>::new(event, events)
                .simultaneous_action_manager(self.state),
        )
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
