use cgmath::Vector2;
use ggez::{graphics, Context, GameResult};

use super::pawn::{Pawn, PawnType};

#[derive(Clone, PartialEq, Debug)]
pub struct Player {
  image: graphics::Image,
  ptype: PawnType,
  position: Vector2<i32>,
}

impl Player {
  pub fn new(ctx: &mut Context, position: Vector2<i32>) -> Self {
    let image = graphics::Image::new(ctx, "/images/player.png").unwrap();

    Player {
      image,
      ptype: PawnType::Player,
      position,
    }
  }

  pub fn update() {}
}

impl Pawn for Player {
  fn get_type(&self) -> PawnType {
    self.ptype
  }

  fn set_position(&mut self, position: Vector2<i32>) {
    self.position = position;
  }

  fn get_position(&self) -> Vector2<i32> {
    self.position
  }

  fn draw(&self, ctx: &mut Context) -> GameResult {
    graphics::draw(
      ctx,
      &self.image,
      (ggez::mint::Point2 {
        x: (self.position.x as f32 * 64.),
        y: (self.position.y as f32 * 64.),
      },),
    )?;

    Ok(())
  }
}
