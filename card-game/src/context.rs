use card_stack::priority::GetState;
use state_validation::{StateFilter, ValidAction, ValidationError, Validator};

pub struct HistoricalContext<History: ActionHistory, State> {
    history: History,
    state: State,
}

impl<History: ActionHistory, State> HistoricalContext<History, State> {
    pub fn new(history: History, state: State) -> Self {
        HistoricalContext { history, state }
    }
    pub fn validate<Input, Filter: StateFilter<State, Input>>(
        self,
        input: Input,
    ) -> Result<ValidContext<History, State, Input, Filter>, ValidationError<State, Filter::Error>>
    {
        Ok(ValidContext {
            history: self.history,
            validator: Validator::try_new(self.state, input)?,
        })
    }
}

pub struct ValidContext<History: ActionHistory, State, Input, Filter: StateFilter<State, Input>> {
    history: History,
    validator: Validator<State, Input, Filter>,
}

impl<History: ActionHistory, State, Input, Filter: StateFilter<State, Input>>
    ValidContext<History, State, Input, Filter>
{
    pub fn action<
        Action: ValidAction<State, Input, Filter = Filter>
            + ActionInfo<State, Filter::ValidOutput, History::History>,
    >(
        mut self,
        action: Action,
    ) -> HistoricalContext<History, Action::Output> {
        self.history
            .push(action.info(self.validator.state(), self.validator.valid_output()));
        HistoricalContext::new(self.history, self.validator.execute(action))
    }
}

pub trait ActionHistory {
    type History;
    fn push(&mut self, history: Self::History);
}

pub trait ActionInfo<State, Input, Info> {
    fn info(&self, state: &State, input: &Input) -> Info;
}
