/*use state_validation::ValidAction;

pub struct Passive {}

pub struct ReplacementEffects<State, Input> {
    required_effects: Vec<Box<dyn ValidAction<State, Input>>>,
    optional_effects: Vec<Box<dyn ValidAction<State, Input>>>,
}

impl<State, Input> ReplacementEffects<State, Input> {
    pub fn required_replacement(&mut self, replacement_action: impl ValidAction<State, Input>) {
        self.required_effects.push(Box::new(replacement_action))
    }
    pub fn optional_replacement(&mut self, replacement_action: impl ValidAction<State, Input>) {
        self.optional_effects.push(Box::new(replacement_action))
    }
}*/

/*use card_stack::priority::{GetState, PriorityMut};
use state_validation::ValidAction;

use crate::events::{EventListener, EventListenerConstructor};

pub struct SharedState<State> {
    state: State,
    into_state:
        Box<dyn ReconstructFromSharedState<State = Box<dyn std::any::Any>, SharedState = State>>,
}
impl<State> SealedTrait for SharedState<State> {}
impl<State> FromSharedState for SharedState<State> {}
trait SealedTrait {}
pub trait FromSharedState: SealedTrait {
    type State;
    fn into_state(self) -> Self::State;
}
pub trait SharedStateReconstructor<SharedState> {
    type Reconstructor;
    fn shared_state(self) -> (SharedState, Self::Reconstructor);
}
pub trait ReconstructFromSharedState {
    type State;
    type SharedState;
    fn into_state(self, shared_state: Self::SharedState) -> Self::State;
}

pub struct SharedStateAbility<Ability, SharedState> {
    ability: Ability,
    _m: std::marker::PhantomData<SharedState>,
}

impl<Ability, SharedState> SharedStateAbility<Ability, SharedState> {
    const fn new(ability: Ability) -> Self {
        SharedStateAbility {
            ability,
            _m: std::marker::PhantomData,
        }
    }
}

impl<SharedState, State, Event, Ability> EventListenerConstructor<State, Event>
    for SharedStateAbility<Ability, SharedState>
where
    Ability: EventListenerConstructor<SharedState, Event>,
    Event:
        crate::events::Event<PriorityMut<SharedState>> + crate::events::Event<PriorityMut<State>>,
{
    type Input = <Ability as EventListenerConstructor<SharedState, Event>>::Input;
    fn new_listener(
        source_card_id: crate::identifications::SourceCardID,
        input: Self::Input,
    ) -> Self {
        SharedStateAbility::new(Ability::new_listener(source_card_id, input))
    }
}

impl<SharedState, State, Event, Ability> EventListener<State, Event>
    for SharedStateAbility<Ability, SharedState>
where
    Ability: EventListener<SharedState, Event> + Clone + Send + Sync + 'static,
    State: GetState<SharedState>,
    Event:
        crate::events::Event<PriorityMut<SharedState>> + crate::events::Event<PriorityMut<State>>,
{
    type Filter = <Self as EventListener<SharedState, Event>>::Filter;
    type FilterInput = <Self as EventListener<SharedState, Event>>::FilterInput;
    fn filter_input(&self, event: &Event) -> Self::FilterInput {
        <Self as EventListener<SharedState, Event>>::filter_input(self, event)
    }
    type Action =
        SharedStateAbility<<Self as EventListener<SharedState, Event>>::Action, SharedState>;
    type ActionInput = <Self as EventListener<SharedState, Event>>::ActionInput;
    fn action(
        &mut self,
        state: &State,
        event: &Event,
        value: <Self::Filter as crate::events::EventStateFilter<State, Self::FilterInput>>::ValidOutput,
    ) -> (Self::Action, Self::ActionInput) {
        let (action, action_input) = <Ability as EventListener<SharedState, Event>>::action(
            &mut self.ability,
            state.state(),
            event,
            value,
        );
        (SharedStateAbility::new(action), action_input)
    }
}

impl<SharedState, State, Input, Ability> ValidAction<State, Input>
    for SharedStateAbility<Ability, SharedState>
where
    Ability: ValidAction<self::SharedState<SharedState>, Input>,
    State: self::SharedStateReconstructor<SharedState>,
    State::Remainder: StateReconstruction,
{
    type Filter = <Self as ValidAction<self::SharedState<SharedState>, Input>>::Filter;
    type Output = <Self as ValidAction<self::SharedState<SharedState>, Input>>::Output;
    fn with_valid_input(
        self,
        state: State,
        valid: <Self::Filter as state_validation::StateFilter<State, Input>>::ValidOutput,
    ) -> Self::Output {
        let (shared_state, state_reconstructor) = state.shared_state();
        <Self as ValidAction<SharedState, Input>>::with_valid_input(self, shared_state, valid);
    }
}*/
