use cgmath::Vector2;
use ggez::graphics::Image;
use ggez::{graphics, Context, GameResult};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PawnType {
  Player,
  Ground,
  GBox(usize),
  Block,
  Place,
}

#[derive(Clone)]
pub struct Pawn {
  image: Image,
  ptype: PawnType,
  position: Vector2<f32>,
  cell_size: (usize, usize),
}

impl Pawn {
  pub fn new(
    ctx: &mut Context,
    image_name: String,
    ptype: PawnType,
    position: Vector2<f32>,
    cell_size: (usize, usize),
  ) -> Self {
    let image = graphics::Image::new(ctx, format!("/images/{}", image_name)).unwrap();
    Pawn {
      image,
      ptype,
      position,
      cell_size,
    }
  }

  pub fn set_position(&mut self, position: Vector2<f32>) {
    self.position = position;
  }

  pub fn get_position(&self) -> Vector2<f32> {
    self.position
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult {
    graphics::draw(
      ctx,
      &self.image,
      (ggez::mint::Point2 {
        x: (self.position.x * self.cell_size.0 as f32),
        y: (self.position.y * self.cell_size.1 as f32),
      },),
    )?;

    Ok(())
  }
}
