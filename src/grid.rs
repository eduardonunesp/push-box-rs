use ggez::graphics;

use super::direction::Direction;

pub const GRID_SIZE: (i16, i16) = (30, 20);
pub const GRID_CELL_SIZE: (i16, i16) = (32, 32);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
  x: i16,
  y: i16,
}

trait ModuloSigned {
  fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where
  T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
  fn modulo(&self, n: T) -> T {
    // Because of our trait bounds, we can now apply these operators.
    (self.clone() % n.clone() + n.clone()) % n.clone()
  }
}

impl GridPosition {
  pub fn new(x: i16, y: i16) -> Self {
    GridPosition { x, y }
  }
}

impl From<GridPosition> for graphics::Rect {
  fn from(pos: GridPosition) -> Self {
    graphics::Rect::new_i32(
      pos.x as i32 * GRID_CELL_SIZE.0 as i32,
      pos.y as i32 * GRID_CELL_SIZE.1 as i32,
      GRID_CELL_SIZE.0 as i32,
      GRID_CELL_SIZE.1 as i32,
    )
  }
}

impl From<(i16, i16)> for GridPosition {
  fn from(pos: (i16, i16)) -> Self {
    GridPosition { x: pos.0, y: pos.1 }
  }
}

#[derive(Copy, Clone)]
pub enum CellType {
  Empty = 0,
  Player,
  Ground,
  Box,
  Block,
  Place,
}

pub struct Grid {
  positions: Vec<Vec<i32>>,
}

impl Grid {
  pub fn new(x: i32, y: i32) -> Self {
    let mut positions: Vec<Vec<i32>> = vec![vec![]];

    for _ in 0..x {
      for _ in 0..y {
        positions.push(vec![CellType::Empty as i32]);
      }
    }

    Grid { positions }
  }

  pub fn request_move() {}
}
