use std::env;
use std::path;

use ggez;
use ggez::{event, GameResult};

mod board;
mod gamestate;
mod levels;
mod pawn;

const GRID_SIZE: (usize, usize) = (10, 10);
const GRID_CELL_SIZE: (usize, usize) = (64, 64);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) =
        &mut ggez::ContextBuilder::new("push-box-rs", "eduardonunesp@gmail.com")
            .window_setup(ggez::conf::WindowSetup::default().title("push-box-rs"))
            .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
            .backend(ggez::conf::Backend::OpenGL { major: 3, minor: 2 })
            .add_resource_path(resource_dir)
            .build()?;

    let state = &mut gamestate::GameState::new(ctx, GRID_SIZE, GRID_CELL_SIZE)?;
    state.next_stage(ctx);
    event::run(ctx, events_loop, state)
}
