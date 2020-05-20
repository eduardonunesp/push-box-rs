use cgmath::Vector2;
use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};

use super::board::Board;

pub struct GameState {
    board: Board,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut board = Board::new(ctx, 10, 10);
        Ok(GameState { board })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.board.update(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.board.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }

        self.board.key_down(keycode);
    }
}
