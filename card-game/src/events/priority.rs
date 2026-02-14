use card_stack::priority::{Priority, PriorityMut, PriorityStack};

use crate::events::{Event, EventAction, EventManagerState, GetEventManager};

pub struct EventPriorityStack<
    State: EventManagerState,
    Ev: Event<PriorityMut<Priority<State::State>>>,
>(
    PriorityStack<
        State::State,
        EventAction<Priority<State::State>, Ev, <State::State as GetEventManager<Ev>>::Output>,
    >,
)
where
    State::State: GetEventManager<Ev>;

impl<State: EventManagerState, Ev: Event<PriorityMut<Priority<State::State>>>>
    EventPriorityStack<State, Ev>
where
    State::State: GetEventManager<Ev>,
{
    pub fn stack(mut self, stack_action: impl Into<IncitingAction::Stackable>) -> Self {
        let priority_stack = self.0.stack(stack_action.into());
        EventPriorityStack(priority_stack)
    }
    pub fn priority(&self) -> &Priority<State::State> {
        self.0.priority()
    }
    pub fn state(&self) -> &State::State {
        self.0.state()
    }
}
