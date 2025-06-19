mod game;
mod grid;
mod pathfinder;
mod ui;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
