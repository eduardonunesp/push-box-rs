use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer, Context, GameResult};

use super::direction::Direction;
use super::grid::GRID_SIZE;
use super::grid::{Grid, GridPosition};
use super::player::Player;

pub struct GameState {
    player: Player,
    grid: Grid,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        // let player_pos:GridPosition = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        Ok(GameState {
            grid: Grid::new(10, 10),
            player: Player::new(ctx),
        })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            // println!("Delta frame time: {:?} ", timer::delta(ctx));
            // println!("Average FPS: {}", timer::fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.player.draw(ctx)?;
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

        if let Some(dir) = Direction::from_keycode(keycode) {
            // self.grid.request_move(self.dir, dir)
        }
    }
}
