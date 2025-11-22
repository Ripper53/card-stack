use card_stack::priority::GetState;
use state_validation::{StateFilter, ValidAction, Validator};

use crate::{
    ActionHistory, ActionInfo,
    cards::{Card, CardID},
    identifications::{MutID, ValidCardID},
};

pub struct CardCommandManager<F> {
    card_id: ValidCardID<F>,
}

impl<F> CardCommandManager<F> {
    pub fn new(card_id: ValidCardID<F>) -> Self {
        CardCommandManager { card_id }
    }
    pub fn with_history<History>(
        self,
        history: ActionHistory<History>,
    ) -> CardCommandManagerWithHistory<F, History> {
        CardCommandManagerWithHistory {
            card_id: self.card_id,
            history,
        }
    }
}

pub struct CardCommandManagerWithHistory<F, History> {
    card_id: ValidCardID<F>,
    history: ActionHistory<History>,
}

impl<F, History> CardCommandManagerWithHistory<F, History> {
    pub fn validate<State, Filter: StateFilter<State, MutID<ValidCardID<F>>>>(
        self,
        state: State,
    ) -> Result<CardCommand<History, State, MutID<ValidCardID<F>>, Filter>, Filter::Error> {
        Ok(CardCommand {
            history: self.history,
            validator: Validator::try_new(state, MutID::new(self.card_id))?,
        })
    }
    pub fn validate_with_input<State, Input, Filter: StateFilter<State, Input>>(
        self,
        state: State,
        get_input: impl FnOnce(MutID<ValidCardID<F>>) -> Input,
    ) -> Result<CardCommand<History, State, Input, Filter>, Filter::Error> {
        Ok(CardCommand {
            history: self.history,
            validator: Validator::try_new(state, get_input(MutID::new(self.card_id)))?,
        })
    }
}

pub struct CardCommand<History, State, Input, Filter: StateFilter<State, Input>> {
    history: ActionHistory<History>,
    validator: Validator<State, Input, Filter>,
}

impl<History, State, Input, Filter: StateFilter<State, Input>>
    CardCommand<History, State, Input, Filter>
{
    pub fn execute<
        Action: ValidAction<State, Input, Filter = Filter> + ActionInfo<History, Filter::ValidOutput>,
    >(
        mut self,
        action: Action,
    ) -> Action::Output {
        self.history.push(action.info(self.validator.valid_value()));
        self.validator.execute(action)
    }
}
