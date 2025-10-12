use std::collections::HashMap;

use card_game::stack::priority::{GetState, Priority};

use crate::identifications::CharacterID;

pub struct Character {
    pub health: usize,
}

pub struct Game {
    pub characters: HashMap<CharacterID, Character>,
}
pub trait GetStateMut<State>: GetState<State> {
    fn state_mut(&mut self) -> &mut State;
}
impl GetState<Game> for Game {
    fn state(&self) -> &Game {
        self
    }
}
impl GetStateMut<Game> for Game {
    fn state_mut(&mut self) -> &mut Game {
        self
    }
}

impl Game {
    pub fn start() -> StartOfTurnState {
        StartOfTurnState {
            game: Game {
                characters: HashMap::new(),
            },
        }
    }
}

pub struct StartOfTurnState {
    game: Game,
}

pub struct ActionState {
    game: Game,
}

pub struct EndOfTurnState {
    game: Game,
}

macro_rules! impl_turn_state {
    ($step: ident) => {
        impl GetState<Game> for $step {
            fn state(&self) -> &Game {
                &self.game
            }
        }
        impl GetStateMut<Game> for $step {
            fn state_mut(&mut self) -> &mut Game {
                &mut self.game
            }
        }
    };
}
impl_turn_state!(StartOfTurnState);
impl_turn_state!(ActionState);
impl_turn_state!(EndOfTurnState);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        actions::StackDamageAndHeal,
        game::{Character, Game, StartOfTurnState},
        identifications::CharacterID,
        resolvers::{HaltStack, Resolver},
    };
    use card_game::stack::{
        priority::{GetState, Priority, ResolveStack},
        requirements::FulfilledAction,
    };

    #[test]
    fn game() {
        let mut game = Game {
            characters: HashMap::with_capacity(2),
        };
        game.characters
            .insert(CharacterID::new(0), Character { health: 3 });
        game.characters
            .insert(CharacterID::new(1), Character { health: 3 });
        let state = StartOfTurnState { game };
        let priority = Priority::new(state).stack(StackDamageAndHeal::new(CharacterID::new(0)));
        match priority.resolve_next::<Resolver<_>>() {
            ResolveStack::Complete(new_stack) => match new_stack.resolve_next::<Resolver<_>>() {
                ResolveStack::Halt(requirement) => {}
                ResolveStack::Complete(_) => panic!("unexpected path"),
                ResolveStack::Next(_) => panic!("unexpected path"),
            },
            ResolveStack::Next(r) => panic!("unexpected path"),
            ResolveStack::Halt(_) => panic!("unexpected path"),
        }
    }
}
