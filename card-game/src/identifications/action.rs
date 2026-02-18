#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct ActionID(&'static str);
impl ActionID {
    pub fn new(id: &'static str) -> Self {
        ActionID(id)
    }
}

pub trait ActionIdentifier {
    fn action_id() -> ActionID;
}
pub trait ActionDescription<T> {
    fn description() -> T;
}
