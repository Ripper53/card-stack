use state_validation::{StateFilter, ValidAction, Validator};

pub struct HistoricalContext<History, State> {
    history: ActionHistory<History>,
    state: State,
}

impl<History, State> HistoricalContext<History, State> {
    pub fn new(history: ActionHistory<History>, state: State) -> Self {
        HistoricalContext { history, state }
    }
    pub fn validate<Input, Filter: StateFilter<State, Input>>(
        self,
        input: Input,
    ) -> Result<ValidContext<History, State, Input, Filter>, Filter::Error> {
        Ok(ValidContext {
            history: self.history,
            validator: Validator::try_new(self.state, input)?,
        })
    }
}

pub struct ValidContext<History, State, Input, Filter: StateFilter<State, Input>> {
    history: ActionHistory<History>,
    validator: Validator<State, Input, Filter>,
}

impl<History, State, Input, Filter: StateFilter<State, Input>>
    ValidContext<History, State, Input, Filter>
{
    pub fn action<
        Action: ValidAction<State, Input, Filter = Filter> + ActionInfo<History, Filter::ValidOutput>,
    >(
        mut self,
        action: Action,
    ) -> HistoricalContext<History, Action::Output> {
        self.history
            .0
            .push(action.info(self.validator.valid_value()));
        HistoricalContext::new(self.history, self.validator.execute(action))
    }
}

pub struct ActionHistory<History>(Vec<History>);
impl<History> ActionHistory<History> {
    pub fn new() -> Self {
        ActionHistory(Vec::new())
    }
    pub(crate) fn push(&mut self, history: History) {
        self.0.push(history)
    }
}
impl<History> IntoIterator for ActionHistory<History> {
    type Item = History;
    type IntoIter = std::vec::IntoIter<History>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a, History> IntoIterator for &'a ActionHistory<History> {
    type Item = &'a History;
    type IntoIter = std::slice::Iter<'a, History>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

pub trait ActionInfo<Info, Input> {
    fn info(&self, input: &Input) -> Info;
}
