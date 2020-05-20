use std::env;
use std::path;

use ggez;
use ggez::{event, GameResult};

mod block;
mod board;
mod gamestate;
mod gbox;
mod ground;
mod pawn;
mod place;
mod player;

const SCREEN_WIDTH: f32 = 640.;
const SCREEN_HEIGHT: f32 = 640.;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("PushBox", "Some Famous Clone")
        .window_setup(ggez::conf::WindowSetup::default().title("PushBox"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .backend(ggez::conf::Backend::OpenGL { major: 3, minor: 2 })
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut gamestate::GameState::new(ctx)?;
    event::run(ctx, events_loop, state)
}
