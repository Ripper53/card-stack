use state_validation::{CollectedInputs, StateFilter, StateFilterInput};

use crate::{
    actions::ActionSource,
    priority::{Priority, PriorityMut, PriorityStack},
};

pub trait ActionRequirement<State, Input: StateFilterInput, Action: crate::actions::ActionSource> {
    /// Satisfy the requirement.
    type Filter: StateFilter<State, Input>;
    /// All the possible inputs for the requirement (but does not necessarily pass the filter).
    fn collect_inputs(state: &State) -> CollectedInputs<State, impl Iterator<Item = Input>>;
}

pub struct FulfilledAction<Action: crate::actions::ActionSource, Value> {
    action: Action,
    source: Action::Source,
    value: Value,
}

impl<Action: crate::actions::ActionSource, Value> FulfilledAction<Action, Value> {
    pub const fn new(action: Action, source: Action::Source, value: Value) -> Self {
        FulfilledAction {
            action,
            source,
            value,
        }
    }
    pub fn take_action(self) -> Action {
        self.action
    }
    pub fn source(&self) -> &Action::Source {
        &self.source
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl<Action: crate::actions::ActionSource, Value: Send + Sync> ActionSource
    for FulfilledAction<Action, Value>
{
    type Source = Action::Source;
}

pub struct RequirementAction<
    Priority,
    Input: StateFilterInput,
    Action: crate::actions::ActionSource,
> {
    priority: Priority,
    action: Action,
    source: Action::Source,
    _m: std::marker::PhantomData<Input>,
}

impl<Priority, Input: StateFilterInput, Action: crate::actions::ActionSource>
    RequirementAction<Priority, Input, Action>
{
    pub fn priority(&self) -> &Priority {
        &self.priority
    }
    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn source(&self) -> &Action::Source {
        &self.source
    }
}
impl<State, Input: StateFilterInput, Action: crate::actions::IncitingAction<State, Input>>
    RequirementAction<Priority<State>, Input, Action>
where
    Action::Requirement: ActionRequirement<Priority<State>, Input, Action>,
{
    /// If the current state has any inputs that fit the requirement,
    /// return `Some`, otherwise `None`.
    pub fn try_new(
        priority: Priority<State>,
        action: Action,
        source: Action::Source,
    ) -> Option<RequirementAction<Priority<State>, Input, Action>> {
        let collected_inputs = <Action::Requirement as ActionRequirement<
            Priority<State>,
            Input,
            Action,
        >>::collect_inputs(&priority);
        if collected_inputs.fits_any::<<Action::Requirement as ActionRequirement<
            Priority<State>,
            Input,
            Action,
        >>::Filter>(&priority)
        {
            Some(RequirementAction {
                priority,
                action,
                source,
                _m: std::marker::PhantomData::default(),
            })
        } else {
            None
        }
    }
    pub fn select(
        self,
        value: Input,
    ) -> Result<
        Action::Resolved,
        RequirementActionSelectionError<
            Self,
            <<Action::Requirement as ActionRequirement<Priority<State>, Input, Action>>::Filter as StateFilter<Priority<State>, Input>>::Error,
        >,
    >{
        let result = <<Action::Requirement as ActionRequirement<Priority<State>, Input, Action>>::Filter as StateFilter<Priority<State>, Input>>::filter(
            &self.priority,
            value,
        );
        match result {
            Ok(input) => Ok(self
                .action
                .resolve(PriorityMut::<Priority<State>>::new(self.priority), input)),
            Err(error) => Err(RequirementActionSelectionError {
                action: self,
                error,
            }),
        }
    }
}
impl<
    State,
    Input: StateFilterInput,
    IncitingAction: crate::actions::IncitingStackable<State>,
    Action: crate::actions::StackAction<State, Input, IncitingAction>,
> RequirementAction<PriorityStack<State, IncitingAction>, Input, Action>
where
    Action::Requirement: ActionRequirement<PriorityStack<State, IncitingAction>, Input, Action>,
{
    /// If the current state has any inputs that fit the requirement,
    /// return `Some`, otherwise `None`.
    pub fn try_new(
        priority: PriorityStack<State, IncitingAction>,
        action: Action,
        source: Action::Source,
    ) -> Option<RequirementAction<PriorityStack<State, IncitingAction>, Input, Action>> {
        let collected_inputs = <Action::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
            Action,
        >>::collect_inputs(&priority);
        if collected_inputs.fits_any::<<Action::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
            Action,
        >>::Filter>(&priority)
        {
            Some(RequirementAction {
                priority,
                action,
                source,
                _m: std::marker::PhantomData::default(),
            })
        } else {
            None
        }
    }
    pub fn select(
        self,
        value: Input,
    ) -> Result<
        Action::Resolved,
        RequirementActionSelectionError<
            Self,
            <<Action::Requirement as ActionRequirement<
                PriorityStack<State, IncitingAction>,
                Input,
                Action,
            >>::Filter as StateFilter<PriorityStack<State, IncitingAction>, Input>>::Error,
        >,
    > {
        let result = <<Action::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
            Action,
        >>::Filter as StateFilter<PriorityStack<State, IncitingAction>, Input>>::filter(
            &self.priority,
            value,
        );
        match result {
            Ok(input) => Ok(self.action.resolve(
                PriorityMut::<PriorityStack<State, IncitingAction>>::new(self.priority),
                input,
            )),
            Err(error) => Err(RequirementActionSelectionError {
                action: self,
                error,
            }),
        }
    }
}
pub struct RequirementActionSelectionError<Action, E: std::error::Error> {
    action: Action,
    error: E,
}
impl<Action, E: std::error::Error> RequirementActionSelectionError<Action, E> {
    pub fn take_requirement_action(self) -> Action {
        self.action
    }
    pub fn take_all(self) -> (Action, E) {
        (self.action, self.error)
    }
    pub fn error(&self) -> &E {
        &self.error
    }
}
