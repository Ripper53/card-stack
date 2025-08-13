use crate::{
    actions::{ActionSource, NeverError},
    priority::{
        IncitingPriority, Priority, PriorityError, PriorityMut, PriorityStack, StackPriority,
    },
};

pub trait SatisfyRequirement<Priority>: Send + Sync {
    type Value: Send + Sync;
    type RequirementError: std::error::Error + Send + Sync;
    /// Satisfy the requirement.
    fn satisfy(
        &self,
        priority: &Priority,
        value: &Self::Value,
    ) -> Result<(), Self::RequirementError>;
    /// Is there only one available selection,
    /// and if there is, what is it?
    fn single_selection(&self, priority: &Priority) -> Option<Self::Value>;
    /// Force a selection, can be used when turn's time runs out.
    /// `ActionRequirement::can_satisfy` is always called before this function.
    fn force_selection(&self, priority: &Priority) -> Self::Value;
}
impl<Priority> SatisfyRequirement<Priority> for () {
    type Value = ();
    type RequirementError = NeverError;
    fn satisfy(
        &self,
        _priority: &Priority,
        _value: &Self::Value,
    ) -> Result<(), Self::RequirementError> {
        Ok(())
    }
    fn force_selection(&self, _priority: &Priority) -> Self::Value {}
    fn single_selection(&self, _priority: &Priority) -> Option<Self::Value> {
        Some(())
    }
}
impl<Priority, R0: SatisfyRequirement<Priority>, R1: SatisfyRequirement<Priority>>
    SatisfyRequirement<Priority> for (R0, R1)
{
    type Value = (R0::Value, R1::Value);
    type RequirementError = TwoSatisfyError<R0::RequirementError, R1::RequirementError>;
    fn satisfy(
        &self,
        priority: &Priority,
        (value_0, value_1): &Self::Value,
    ) -> Result<(), Self::RequirementError> {
        if let Err(e) = self.0.satisfy(priority, value_0) {
            Err(TwoSatisfyError::First(e))
        } else if let Err(e) = self.1.satisfy(priority, value_1) {
            Err(TwoSatisfyError::Second(e))
        } else {
            Ok(())
        }
    }
    fn force_selection(&self, priority: &Priority) -> Self::Value {
        (
            self.0.force_selection(priority),
            self.1.force_selection(priority),
        )
    }
    fn single_selection(&self, priority: &Priority) -> Option<Self::Value> {
        if let Some(value_0) = self.0.single_selection(priority) {
            if let Some(value_1) = self.1.single_selection(priority) {
                Some((value_0, value_1))
            } else {
                None
            }
        } else {
            None
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum TwoSatisfyError<E0: std::error::Error, E1: std::error::Error> {
    #[error(transparent)]
    First(E0),
    #[error(transparent)]
    Second(E1),
}
pub trait ActionRequirement<Priority, Action: crate::actions::ActionSource>: Send + Sync {
    type Satisfy: SatisfyRequirement<Priority>;
    type RequirementError: std::error::Error + Send + Sync;
    /// Can this requirement even be satisfied?
    ///
    /// **TIP:** use `SatisfyRequirement::satisfy` within this function to check if it is compatible.
    fn can_satisfy(
        &self,
        priority: Priority,
        action: Action,
        source: Action::Source,
    ) -> Result<
        RequirementAction<Priority, Action, Self::Satisfy>,
        PriorityError<Priority, Self::RequirementError>,
    >;
}
impl<Priority, Action: crate::actions::ActionSource> ActionRequirement<Priority, Action> for () {
    type Satisfy = ();
    type RequirementError = NeverError;
    fn can_satisfy(
        &self,
        priority: Priority,
        action: Action,
        source: Action::Source,
    ) -> Result<
        RequirementAction<Priority, Action, Self::Satisfy>,
        PriorityError<Priority, Self::RequirementError>,
    > {
        Ok(RequirementAction::new(priority, action, source, ()))
    }
}
impl<
    Priority,
    Action: crate::actions::ActionSource,
    R0: ActionRequirement<Priority, Action>,
    R1: ActionRequirement<Priority, Action>,
> ActionRequirement<Priority, Action> for (R0, R1)
{
    type Satisfy = (R0::Satisfy, R1::Satisfy);
    type RequirementError = TwoRequirementError<R0::RequirementError, R1::RequirementError>;
    fn can_satisfy(
        &self,
        priority: Priority,
        action: Action,
        source: Action::Source,
    ) -> Result<
        RequirementAction<Priority, Action, Self::Satisfy>,
        PriorityError<Priority, Self::RequirementError>,
    > {
        match self.0.can_satisfy(priority, action, source) {
            Ok(requirement_action) => {
                let (priority, action, source, satisfy_0) = requirement_action.take_contents();
                match self.1.can_satisfy(priority, action, source) {
                    Ok(requirement_action) => {
                        let (priority, action, source, satisfy_1) =
                            requirement_action.take_contents();
                        Ok(RequirementAction::new(
                            priority,
                            action,
                            source,
                            (satisfy_0, satisfy_1),
                        ))
                    }
                    Err(e) => Err(PriorityError {
                        priority: e.priority,
                        error: TwoRequirementError::Second(e.error),
                    }),
                }
            }
            Err(e) => Err(PriorityError {
                priority: e.priority,
                error: TwoRequirementError::First(e.error),
            }),
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum TwoRequirementError<E0: std::error::Error, E1: std::error::Error> {
    #[error(transparent)]
    First(E0),
    #[error(transparent)]
    Second(E1),
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

pub struct RequirementAction<Priority, Action: crate::actions::ActionSource, Satisfy> {
    priority: Priority,
    action: Action,
    source: Action::Source,
    satisfy: Satisfy,
}

impl<Priority, Action: crate::actions::ActionSource, Satisfy>
    RequirementAction<Priority, Action, Satisfy>
{
    pub fn new(
        priority: Priority,
        action: Action,
        source: Action::Source,
        satisfy: Satisfy,
    ) -> Self {
        RequirementAction {
            priority,
            action,
            source,
            satisfy,
        }
    }
    pub fn priority(&self) -> &Priority {
        &self.priority
    }
    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn source(&self) -> &Action::Source {
        &self.source
    }
    pub fn satisfy(&self) -> &Satisfy {
        &self.satisfy
    }
    pub(crate) fn take_contents(self) -> (Priority, Action, Action::Source, Satisfy) {
        (self.priority, self.action, self.source, self.satisfy)
    }
}
impl<State, Action: crate::actions::ActionSource, Satisfy: SatisfyRequirement<Priority<State>>>
    RequirementAction<Priority<State>, Action, Satisfy>
{
    pub fn select(
        self,
        value: Satisfy::Value,
    ) -> Result<
        crate::priority::PriorityStack<State, FulfilledAction<Action, Satisfy::Value>>,
        Satisfy::RequirementError,
    >
    where
        FulfilledAction<Action, Satisfy::Value>: crate::actions::IncitingAction<State>,
    {
        if let Err(e) = self.satisfy.satisfy(&self.priority, &value) {
            Err(e)
        } else {
            Ok(PriorityMut::<Priority<State>>::new(self.priority)
                .stack::<FulfilledAction<Action, Satisfy::Value>>(FulfilledAction::new(
                    self.action,
                    self.source,
                    value,
                ))
                .take_priority())
        }
    }
}
impl<
    State,
    IncitingAction: crate::actions::IncitingAction<State>,
    Action: crate::actions::ActionSource,
    Satisfy: SatisfyRequirement<PriorityStack<State, IncitingAction>>,
> RequirementAction<PriorityStack<State, IncitingAction>, Action, Satisfy>
where
    IncitingAction::Stackable: crate::actions::StackAction<State, IncitingAction>,
{
    pub fn select(
        self,
        value: Satisfy::Value,
    ) -> Result<PriorityStack<State, IncitingAction>, Satisfy::RequirementError>
    where
        FulfilledAction<Action, Satisfy::Value>: Into<IncitingAction::Stackable>,
    {
        if let Err(e) = self.satisfy.satisfy(&self.priority, &value) {
            Err(e)
        } else {
            Ok(
                PriorityMut::<PriorityStack<State, IncitingAction>>::new(self.priority)
                    .stack(FulfilledAction::new(self.action, self.source, value).into())
                    .take_priority(),
            )
        }
    }
}
