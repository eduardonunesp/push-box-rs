use cgmath::Vector2;
use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};

use super::board::Board;

pub struct GameState {
    board: Board,
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        grid_size: (usize, usize),
        cell_size: (usize, usize),
    ) -> GameResult<GameState> {
        let mut board = Board::new(ctx, grid_size, cell_size);
        board.set_player_start(Vector2::<f32>::new(2., 5.));
        board.add_box(ctx, Vector2::<f32>::new(5., 5.), cell_size);
        board.add_place(ctx, Vector2::<f32>::new(7., 5.), cell_size);
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
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }

        if !repeat {
            self.board.key_down(keycode);
        }
    }
}
