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
    pub fn with_history<'a, History: ActionHistory + ?Sized>(
        self,
        history: &'a mut History,
    ) -> CommandManagerWithHistory<'a, ID, History> {
        CommandManagerWithHistory {
            id: self.id,
            history,
        }
    }
}

pub struct CommandManagerWithHistory<'a, ID, History: ActionHistory + ?Sized> {
    id: ID,
    history: &'a mut History,
}

impl<'a, ID, History: ActionHistory + ?Sized> CommandManagerWithHistory<'a, ID, History> {
    pub fn validate<State, Filter: StateFilter<State, MutID<ID>>>(
        self,
        state: State,
    ) -> Result<Command<'a, History, State, MutID<ID>, Filter>, ValidationError<State, Filter::Error>>
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
    ) -> Result<Command<'a, History, State, Input, Filter>, ValidationError<State, Filter::Error>>
    {
        Ok(Command {
            history: self.history,
            validator: Validator::try_new(state, get_input(MutID::new(self.id)))?,
        })
    }
}

pub struct Command<
    'a,
    History: ActionHistory + ?Sized,
    State,
    Input,
    Filter: StateFilter<State, Input>,
> {
    history: &'a mut History,
    validator: Validator<State, Input, Filter>,
}

impl<'a, History: ActionHistory + ?Sized, State, Input, Filter: StateFilter<State, Input>>
    Command<'a, History, State, Input, Filter>
{
    pub fn execute<
        Action: ValidAction<State, Input, Filter = Filter>
            + ActionInfo<State, Filter::ValidOutput, History::History>,
    >(
        mut self,
        action: Action,
    ) -> Action::Output {
        self.history
            .push(action.info(self.validator.state(), self.validator.valid_output()));
        self.validator.execute(action)
    }
}
