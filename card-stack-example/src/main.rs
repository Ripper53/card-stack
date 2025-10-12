use crate::game::Game;

mod actions;
mod filters;
mod game;
mod identifications;
mod requirements;
mod resolvers;
mod stack;

fn main() {
    let state = Game::start();
}
