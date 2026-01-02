mod defs;
mod combat;
mod game_state;
mod scenes;
mod assets;
mod player;
mod world;
mod system;
mod input_handler;
mod physics;

use tetra::ContextBuilder;
use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::game_state::GameState;

fn main() -> tetra::Result {
    ContextBuilder::new("Gorkitale", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(false)
        .build()?
        .run(GameState::new)
}
