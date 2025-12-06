use card_stack::priority::GetState;
use state_validation::{StateFilter, ValidAction, ValidationError, Validator};

use crate::{ActionHistory, ActionInfo, identifications::MutID};

pub struct CommandManager<ID> {
    id: ID,
}

impl<ID> CommandManager<ID> {
    pub fn new(id: ID) -> Self {
        CommandManager { id }
    }
    pub fn with_history<History>(
        self,
        history: ActionHistory<History>,
    ) -> CommandManagerWithHistory<ID, History> {
        CommandManagerWithHistory {
            id: self.id,
            history,
        }
    }
}

pub struct CommandManagerWithHistory<ID, History> {
    id: ID,
    history: ActionHistory<History>,
}

impl<ID, History> CommandManagerWithHistory<ID, History> {
    pub fn validate<State, Filter: StateFilter<State, MutID<ID>>>(
        self,
        state: State,
    ) -> Result<Command<History, State, MutID<ID>, Filter>, ValidationError<State, Filter::Error>>
    {
        Ok(Command {
            history: self.history,
            validator: Validator::try_new(state, MutID::new(self.id))?,
        })
    }
    pub fn validate_with_input<State, Input, Filter: StateFilter<State, Input>>(
        self,
        state: State,
        get_input: impl FnOnce(MutID<ID>) -> Input,
    ) -> Result<Command<History, State, Input, Filter>, ValidationError<State, Filter::Error>> {
        Ok(Command {
            history: self.history,
            validator: Validator::try_new(state, get_input(MutID::new(self.id)))?,
        })
    }
}

pub struct Command<History, State, Input, Filter: StateFilter<State, Input>> {
    history: ActionHistory<History>,
    validator: Validator<State, Input, Filter>,
}

impl<History, State, Input, Filter: StateFilter<State, Input>>
    Command<History, State, Input, Filter>
{
    pub fn execute<
        Action: ValidAction<State, Input, Filter = Filter>
            + ActionInfo<State, Filter::ValidOutput, History>,
    >(
        mut self,
        action: Action,
    ) -> Action::Output {
        self.history
            .push(action.info(self.validator.state(), self.validator.valid_output()));
        self.validator.execute(action)
    }
}
