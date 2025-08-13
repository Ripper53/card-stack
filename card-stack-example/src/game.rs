use std::collections::HashMap;

use card_game::stack::priority::GetState;

use crate::identifications::CharacterID;

pub struct Character {
    pub health: usize,
}

pub struct Game {
    pub characters: HashMap<CharacterID, Character>,
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

pub trait TurnState: Send + Sync {
    fn game(&self) -> &Game;
    fn game_mut(&mut self) -> &mut Game;
}
macro_rules! meow {
    ($step: ident) => {
        impl TurnState for $step {
            fn game(&self) -> &Game {
                &self.game
            }
            fn game_mut(&mut self) -> &mut Game {
                &mut self.game
            }
        }
        impl GetState<Game> for $step {
            fn state(&self) -> &Game {
                self.game()
            }
        }
    };
}
meow!(StartOfTurnState);
meow!(ActionState);
meow!(EndOfTurnState);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        actions::{FulfilledStateAction, StackDamageAndHeal},
        game::{Character, Game, StartOfTurnState},
        identifications::CharacterID,
        resolvers::{Resolved, Resolver},
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
        let priority =
            Priority::new(state).stack(FulfilledStateAction::from(FulfilledAction::new(
                StackDamageAndHeal,
                CharacterID::new(0),
                (CharacterID::new(0), CharacterID::new(1)),
            )));
        match priority.resolve_next::<Resolver<_>>() {
            ResolveStack::Next(_) => panic!("unexpected path"),
            ResolveStack::Complete(r) => match r {
                Resolved::Priority(_) => panic!("unexpected path"),
                Resolved::Stack(stack) => {
                    let health = stack
                        .state()
                        .game
                        .characters
                        .get(&CharacterID::new(0))
                        .unwrap()
                        .health;
                    assert_eq!(3, health);
                    let health = stack
                        .state()
                        .game
                        .characters
                        .get(&CharacterID::new(1))
                        .unwrap()
                        .health;
                    assert_eq!(3, health);
                    match stack.resolve_next::<Resolver<_>>() {
                        ResolveStack::Next(stack) => {
                            let health = stack
                                .state()
                                .game
                                .characters
                                .get(&CharacterID::new(0))
                                .unwrap()
                                .health;
                            assert_eq!(3, health);
                            let health = stack
                                .state()
                                .game
                                .characters
                                .get(&CharacterID::new(1))
                                .unwrap()
                                .health;
                            assert_eq!(4, health);
                            match stack.resolve_next::<Resolver<_>>() {
                                ResolveStack::Complete(r) => match r {
                                    Resolved::Priority(priority) => {
                                        let health = priority
                                            .state()
                                            .game
                                            .characters
                                            .get(&CharacterID::new(0))
                                            .unwrap()
                                            .health;
                                        assert_eq!(2, health);
                                        let health = priority
                                            .state()
                                            .game
                                            .characters
                                            .get(&CharacterID::new(1))
                                            .unwrap()
                                            .health;
                                        assert_eq!(4, health);
                                    }
                                    Resolved::Stack(_) => panic!("unexpected path"),
                                },
                                ResolveStack::Next(_) => panic!("unexpected path"),
                            }
                        }
                        ResolveStack::Complete(_) => panic!("unexpected path"),
                    }
                }
            },
        }
    }
}
