use state_validation::{CollectedInputs, StateFilter};

use crate::{
    actions::ActionSource,
    priority::{Priority, PriorityMut, PriorityStack},
};

pub trait ActionRequirement<State, Input> {
    /// Satisfy the requirement.
    type Filter: StateFilter<State, Input>;
    /// All the possible inputs for the requirement (but does not necessarily pass the filter).
    fn collect_inputs(state: &State) -> CollectedInputs<State, impl Iterator<Item = Input>>;
}
struct NoIter<Item>(std::marker::PhantomData<Item>);
impl<Item> Iterator for NoIter<Item> {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
impl<State> ActionRequirement<State, ()> for () {
    type Filter = ();
    fn collect_inputs(_state: &State) -> CollectedInputs<State, impl Iterator<Item = ()>> {
        CollectedInputs::new(NoIter(std::marker::PhantomData::default()))
    }
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
    fn source(&self) -> &Self::Source {
        self.action.source()
    }
}

pub struct RequirementAction<Priority, Input, Action> {
    priority: Priority,
    action: Action,
    _m: std::marker::PhantomData<Input>,
}

impl<Priority, Input, Action> RequirementAction<Priority, Input, Action> {
    pub fn priority(&self) -> &Priority {
        &self.priority
    }
    pub fn action(&self) -> &Action {
        &self.action
    }
}
impl<State, Input, Action: crate::actions::IncitingAction<State, Input>>
    RequirementAction<Priority<State>, Input, Action>
where
    Action::Requirement: ActionRequirement<Priority<State>, Input>,
{
    /// If the current state has any inputs that fit the requirement,
    /// return `Some`, otherwise `None`.
    pub fn try_new(
        priority: Priority<State>,
        action: Action,
    ) -> Result<
        RequirementAction<Priority<State>, Input, Action>,
        TryNewRequirementActionError<Priority<State>, Action>,
    > {
        let collected_inputs =
            <Action::Requirement as ActionRequirement<Priority<State>, Input>>::collect_inputs(
                &priority,
            );
        if collected_inputs
            .fits_any::<<Action::Requirement as ActionRequirement<Priority<State>, Input>>::Filter>(
            &priority,
        ) {
            Ok(RequirementAction {
                priority,
                action,
                _m: std::marker::PhantomData::default(),
            })
        } else {
            Err(TryNewRequirementActionError { priority, action })
        }
    }
    pub fn select(
        self,
        value: Input,
    ) -> Result<
        Action::Resolved,
        RequirementActionSelectionError<
            Self,
            <<Action::Requirement as ActionRequirement<Priority<State>, Input>>::Filter as StateFilter<Priority<State>, Input>>::Error,
        >,
    >{
        let result = <<Action::Requirement as ActionRequirement<Priority<State>, Input>>::Filter as StateFilter<Priority<State>, Input>>::filter(
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
#[derive(thiserror::Error)]
#[error("requirement for action is impossible to fulfill")]
pub struct TryNewRequirementActionError<Priority, Action> {
    pub priority: Priority,
    pub action: Action,
}
impl<Priority, Action> std::fmt::Debug for TryNewRequirementActionError<Priority, Action> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "requirement for action is impossible to fulfill")
    }
}
impl<
    State,
    Input,
    IncitingAction: crate::actions::IncitingActionInfo<State>,
    Action: crate::actions::StackAction<State, Input, IncitingAction>,
> RequirementAction<PriorityStack<State, IncitingAction>, Input, Action>
where
    Action::Requirement: ActionRequirement<PriorityStack<State, IncitingAction>, Input>,
{
    /// If the current state has any inputs that fit the requirement,
    /// return `Some`, otherwise `None`.
    pub fn try_new(
        priority: PriorityStack<State, IncitingAction>,
        action: Action,
    ) -> Result<
        RequirementAction<PriorityStack<State, IncitingAction>, Input, Action>,
        TryNewRequirementActionError<PriorityStack<State, IncitingAction>, Action>,
    > {
        let collected_inputs = <Action::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
        >>::collect_inputs(&priority);
        if collected_inputs.fits_any::<<Action::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
        >>::Filter>(&priority)
        {
            Ok(RequirementAction {
                priority,
                action,
                _m: std::marker::PhantomData::default(),
            })
        } else {
            Err(TryNewRequirementActionError { priority, action })
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
            >>::Filter as StateFilter<PriorityStack<State, IncitingAction>, Input>>::Error,
        >,
    > {
        let result = <<Action::Requirement as ActionRequirement<
            PriorityStack<State, IncitingAction>,
            Input,
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
impl<Action, E: std::error::Error> std::fmt::Debug for RequirementActionSelectionError<Action, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.error, f)
    }
}
impl<Action, E: std::error::Error> std::fmt::Display
    for RequirementActionSelectionError<Action, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, f)
    }
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
