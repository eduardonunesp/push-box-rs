use cgmath::Vector2;
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

use super::ground::Ground;
use super::pawn::{Pawn, PawnType};
use super::player::Player;

const VECTOR_RIGHT: Vector2<i32> = cgmath::Vector2::new(1, 0);
const VECTOR_LEFT: Vector2<i32> = cgmath::Vector2::new(-1, 0);
const VECTOR_DOWN: Vector2<i32> = cgmath::Vector2::new(0, 1);
const VECTOR_UP: Vector2<i32> = cgmath::Vector2::new(0, -1);
const VECTOR_ZERO: Vector2<i32> = cgmath::Vector2::new(0, 0);

pub struct Board {
  player: Player,
  ground: Vec<Vec<Ground>>,
  width: i32,
  height: i32,
  grid: Vec<Vec<PawnType>>,
}

impl Board {
  pub fn new(ctx: &mut Context, width: i32, height: i32) -> Self {
    let grid: Vec<Vec<PawnType>> = vec![vec![PawnType::Ground; 10]; 10];
    let ground: Vec<Vec<Ground>> = vec![vec![Ground::new(ctx, Vector2::<i32>::new(0, 0)); 10]; 10];
    let player = Player::new(ctx, Vector2::<i32>::new(0, 0));

    Board {
      player,
      ground,
      grid,
      width,
      height,
    }
  }

  pub fn update(&mut self, _ctx: &mut Context) {}

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
    for (i, row) in self.ground.iter_mut().enumerate() {
      for (j, col) in row.iter_mut().enumerate() {
        col.set_position(Vector2::<i32>::new(i as i32, j as i32));
        col.draw(ctx)?;
      }
    }

    self.player.draw(ctx)?;
    Ok(())
  }

  pub fn key_down(&mut self, keycode: KeyCode) {
    let dest = match keycode {
      KeyCode::Right => VECTOR_RIGHT,
      KeyCode::Left => VECTOR_LEFT,
      KeyCode::Up => VECTOR_UP,
      KeyCode::Down => VECTOR_DOWN,
      _ => VECTOR_ZERO,
    };

    if let Some(cell_dest) = self.request_move(dest) {
      self.player.set_position(cell_dest);
    }
  }

  fn request_move(&mut self, direction: cgmath::Vector2<i32>) -> Option<Vector2<i32>> {
    let cell_start = self.player.get_position();
    let cell_dest = cell_start + direction;
    Some(cell_dest)
  }

  fn set_cell(&mut self, row: i32, col: i32) {
    assert!(row < self.width);
    assert!(col < self.height);
  }
}
