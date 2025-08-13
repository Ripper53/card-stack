use card_game::stack::priority::{Priority, PriorityStack};

use crate::{
    actions::{ResolvedIncitingAction, ResolvedStackAction},
    game::TurnState,
};

pub struct Resolver<T>(std::marker::PhantomData<T>);

impl<
    T: card_game::stack::actions::IncitingAction<State>,
    State: TurnState,
    IncitingAction: card_game::stack::actions::IncitingAction<
            State,
            ResolvedIncitingAction = ResolvedIncitingAction<State, T>,
        >,
> card_game::stack::priority::Resolver<State, IncitingAction> for Resolver<T>
where
    IncitingAction::Stackable: card_game::stack::actions::StackAction<
            State,
            IncitingAction,
            ResolvedStackAction = ResolvedStackAction<State, IncitingAction>,
        >,
{
    type Resolved = Resolved<State, T>;
    fn resolve_inciting(action: IncitingAction::ResolvedIncitingAction) -> Self::Resolved {
        match action {
            ResolvedIncitingAction::Complete(priority) => Resolved::Priority(priority),
            ResolvedIncitingAction::Continue(stack) => Resolved::Stack(stack),
        }
    }
    fn resolve_stack(
        action: <IncitingAction::Stackable as card_game::stack::actions::StackAction<
            State,
            IncitingAction,
        >>::ResolvedStackAction,
    ) -> card_game::stack::priority::Resolve<PriorityStack<State, IncitingAction>, Self::Resolved>
    {
        match action {
            ResolvedStackAction::Continue(priority) => {
                card_game::stack::priority::Resolve::Continue(priority)
            }
        }
    }
}

pub enum Resolved<State, IncitingAction: card_game::stack::actions::IncitingAction<State>> {
    Priority(Priority<State>),
    Stack(PriorityStack<State, IncitingAction>),
}
