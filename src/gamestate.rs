use cgmath::Vector2;
use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};

use super::board::Board;
use super::levels;

pub struct GameState {
    board: Board,
    current_stage: usize,
    grid_size: (usize, usize),
    cell_size: (usize, usize),
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        grid_size: (usize, usize),
        cell_size: (usize, usize),
    ) -> GameResult<GameState> {
        Ok(GameState {
            grid_size,
            cell_size,
            current_stage: 0,
            board: Board::new(ctx, grid_size, cell_size),
        })
    }

    pub fn next_stage(&mut self, ctx: &mut Context) {
        if (self.current_stage + 1) > levels::LEVELS.len() {
            println!("Thank You!");
            event::quit(ctx);
            return;
        }

        self.board = Board::new(ctx, self.grid_size, self.cell_size);

        for (i, row) in levels::LEVELS[self.current_stage].iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == levels::B {
                    self.board.add_block(
                        ctx,
                        Vector2::<f32>::new(j as f32, i as f32),
                        self.cell_size,
                    );
                }

                if *col == levels::A {
                    self.board
                        .set_player_start(Vector2::<f32>::new(j as f32, i as f32));
                }

                if *col == levels::X {
                    self.board.add_box(
                        ctx,
                        Vector2::<f32>::new(j as f32, i as f32),
                        self.cell_size,
                    );
                }

                if *col == levels::P {
                    self.board.add_place(
                        ctx,
                        Vector2::<f32>::new(j as f32, i as f32),
                        self.cell_size,
                    );
                }
            }
        }

        self.current_stage += 1;
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.board.update(ctx);
            if self.board.winner {
                self.next_stage(ctx);
            }
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
