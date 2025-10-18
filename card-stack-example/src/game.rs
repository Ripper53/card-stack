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
        game::{Character, Game, GetStateMut, StartOfTurnState},
        identifications::CharacterID,
        resolvers::{HaltStack, NoInput},
        stack::RemoveCharacters,
    };
    use card_game::stack::{
        actions::IncitingActionInfo,
        priority::{GetState, Priority, PriorityStack, ResolveStack, Resolver},
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
        match priority.resolve_next::<crate::resolvers::Resolver>() {
            ResolveStack::Complete(new_stack) => {
                match new_stack.resolve_next::<crate::resolvers::Resolver>() {
                    ResolveStack::Halt(requirement) => match requirement {
                        HaltStack::DealDamageRequirement(deal_damage) => {
                            let priority = deal_damage.select(CharacterID::new(0)).unwrap();
                            assert_eq!(
                                priority
                                    .state()
                                    .characters
                                    .get(&CharacterID::new(0))
                                    .unwrap()
                                    .health,
                                2
                            );
                            assert_eq!(
                                priority
                                    .state()
                                    .characters
                                    .get(&CharacterID::new(1))
                                    .unwrap()
                                    .health,
                                3
                            );
                            match priority.resolve_next::<crate::resolvers::Resolver>() {
                                ResolveStack::Complete(r) => match r {
                                    Ok(r) => {
                                        let priority = r.select(CharacterID::new(0)).unwrap();
                                        assert_eq!(
                                            priority
                                                .state()
                                                .characters
                                                .get(&CharacterID::new(0))
                                                .unwrap()
                                                .health,
                                            3
                                        );
                                        assert_eq!(
                                            priority
                                                .state()
                                                .characters
                                                .get(&CharacterID::new(1))
                                                .unwrap()
                                                .health,
                                            3
                                        );
                                    }
                                    Err(_) => unreachable!(),
                                },
                                ResolveStack::Next(_) => unreachable!(),
                                ResolveStack::Halt(_) => unreachable!(),
                            }
                        }
                        HaltStack::HealRequirement(_) => unreachable!(),
                    },
                    ResolveStack::Complete(_) => unreachable!(),
                    ResolveStack::Next(_) => unreachable!(),
                }
            }
            ResolveStack::Next(_) => unreachable!(),
            ResolveStack::Halt(_) => unreachable!(),
        }
    }

    #[test]
    fn game_remove_requirements() {
        let mut game = Game {
            characters: HashMap::with_capacity(2),
        };
        game.characters
            .insert(CharacterID::new(0), Character { health: 3 });
        game.characters
            .insert(CharacterID::new(1), Character { health: 3 });
        let state = StartOfTurnState { game };
        let priority = Priority::new(state).stack(StackDamageAndHeal::new(CharacterID::new(0)));
        match priority.resolve_next::<crate::resolvers::Resolver>() {
            ResolveStack::Complete(new_stack) => {
                match new_stack.resolve_next::<crate::resolvers::Resolver>() {
                    ResolveStack::Halt(requirement) => match requirement {
                        HaltStack::DealDamageRequirement(deal_damage) => {
                            let priority = deal_damage.select(CharacterID::new(0)).unwrap();
                            assert_eq!(
                                priority
                                    .state()
                                    .characters
                                    .get(&CharacterID::new(0))
                                    .unwrap()
                                    .health,
                                2,
                            );
                            assert_eq!(
                                priority
                                    .state()
                                    .characters
                                    .get(&CharacterID::new(1))
                                    .unwrap()
                                    .health,
                                3
                            );
                            let priority =
                                priority.stack(crate::stack::StackAction::RemoveCharacters(
                                    RemoveCharacters::default(),
                                ));
                            let ResolveStack::Next(priority) =
                                priority.resolve_next::<crate::resolvers::Resolver>()
                            else {
                                unreachable!()
                            };
                            match priority.resolve_next::<crate::resolvers::Resolver>() {
                                ResolveStack::Complete(r) => match r {
                                    Ok(_) => unreachable!(),
                                    Err(_) => {
                                        // Expected Path
                                    }
                                },
                                ResolveStack::Next(_) => unreachable!(),
                                ResolveStack::Halt(_) => unreachable!(),
                            }
                        }
                        HaltStack::HealRequirement(_) => unreachable!(),
                    },
                    ResolveStack::Complete(_) => unreachable!(),
                    ResolveStack::Next(_) => unreachable!(),
                }
            }
            ResolveStack::Next(_) => unreachable!(),
            ResolveStack::Halt(_) => unreachable!(),
        }
    }
}
