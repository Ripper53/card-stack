use crate::game::Game;

mod actions;
mod game;
mod identifications;
mod requirements;
mod resolvers;
mod stack;

fn main() {
    let state = Game::start();
}
