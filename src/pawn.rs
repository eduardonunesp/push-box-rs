use cgmath::Vector2;
use ggez::{Context, GameResult};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PawnType {
  Player,
  Ground,
  Box,
  Block,
  Place,
}

pub trait Pawn {
  fn get_type(&self) -> PawnType;
  fn get_position(&self) -> Vector2<i32>;
  fn set_position(&mut self, position: Vector2<i32>);
  fn draw(&self, ctx: &mut Context) -> GameResult;
}
