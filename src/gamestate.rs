use cgmath::Vector2;
use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};

use super::board::Board;

pub struct GameState {
    board: Board,
    cell_size: (usize, usize),
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        grid_size: (usize, usize),
        cell_size: (usize, usize),
    ) -> GameResult<GameState> {
        Ok(GameState {
            cell_size,
            board: Board::new(ctx, grid_size, cell_size),
        })
    }

    pub fn load_stage(&mut self, ctx: &mut Context) {
        let blocks = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 9, 0, 0, 0, 2, 0, 0, 3, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        for (i, row) in blocks.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 1 {
                    self.board.add_block(
                        ctx,
                        Vector2::<f32>::new(j as f32, i as f32),
                        self.cell_size,
                    );
                }

                if *col == 9 {
                    self.board
                        .set_player_start(Vector2::<f32>::new(j as f32, i as f32));
                }

                if *col == 2 {
                    self.board.add_box(
                        ctx,
                        Vector2::<f32>::new(j as f32, i as f32),
                        self.cell_size,
                    );
                }

                if *col == 3 {
                    self.board.add_place(
                        ctx,
                        Vector2::<f32>::new(j as f32, i as f32),
                        self.cell_size,
                    );
                }
            }
        }
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
