use ggez::{graphics, Context, GameResult};

pub struct Player {
  player_image: graphics::Image,
}

impl Player {
  pub fn new(ctx: &mut Context) -> Self {
    let player_image = graphics::Image::new(ctx, "/images/player.png").unwrap();
    Player { player_image }
  }

  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    graphics::draw(
      ctx,
      &self.player_image,
      (ggez::mint::Point2 { x: 0.0, y: 0.0 },),
    )?;
    Ok(())
  }
}
