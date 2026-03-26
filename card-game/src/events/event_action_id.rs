#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EventActionID(usize);

impl EventActionID {
    pub(crate) fn new(value: usize) -> Self {
        EventActionID(value)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct EventActionIDBuilder {
    next_value: usize,
}

impl EventActionIDBuilder {
    pub fn build(&mut self) -> EventActionID {
        let event_action_id = EventActionID(self.next_value);
        self.next_value += 1;
        event_action_id
    }
}
