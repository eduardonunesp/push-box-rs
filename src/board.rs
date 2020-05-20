use cgmath::Vector2;
use ggez::event::KeyCode;
use ggez::{Context, GameResult};

use super::block::Block;
use super::gbox::GBox;
use super::ground::Ground;
use super::pawn::{Pawn, PawnType};
use super::place::Place;
use super::player::Player;

const VECTOR_RIGHT: Vector2<i32> = Vector2::new(1, 0);
const VECTOR_LEFT: Vector2<i32> = Vector2::new(-1, 0);
const VECTOR_DOWN: Vector2<i32> = Vector2::new(0, 1);
const VECTOR_UP: Vector2<i32> = Vector2::new(0, -1);
const VECTOR_ZERO: Vector2<i32> = Vector2::new(0, 0);

pub struct Board {
  player: Player,
  ground: Vec<Vec<Ground>>,
  gbox: Vec<GBox>,
  block: Vec<Block>,
  place: Vec<Place>,
  width: i32,
  height: i32,
  grid: Vec<Vec<PawnType>>,
}

impl Board {
  pub fn new(ctx: &mut Context, width: i32, height: i32) -> Self {
    let grid: Vec<Vec<PawnType>> = vec![vec![PawnType::Ground; 10]; 10];
    let ground: Vec<Vec<Ground>> = vec![vec![Ground::new(ctx, Vector2::<i32>::new(0, 0)); 10]; 10];
    let player = Player::new(ctx, Vector2::<i32>::new(0, 0));

    let gbox1 = Vector2::<i32>::new(5, 5);
    let gbox: Vec<GBox> = vec![GBox::new(ctx, 0, gbox1)];

    let place1 = Vector2::<i32>::new(5, 6);
    let place: Vec<Place> = vec![Place::new(ctx, place1)];

    let mut block: Vec<Block> = vec![];

    for p in [(2, 2)].iter() {
      block.push(Block::new(ctx, Vector2::<i32>::new(p.0, p.1)));
    }

    let mut s = Board {
      player,
      ground,
      place,
      block,
      gbox,
      grid,
      width,
      height,
    };

    for p in [(2, 2)].iter() {
      s.set_cell_type(Vector2::<i32>::new(p.0, p.1), PawnType::Block);
    }

    s.set_cell_type(place1, PawnType::Place);

    s.set_cell_type(gbox1, PawnType::GBox(0));

    s
  }

  pub fn update(&mut self, _ctx: &mut Context) {}

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
    for (i, row) in self.ground.iter_mut().enumerate() {
      for (j, col) in row.iter_mut().enumerate() {
        col.set_position(Vector2::<i32>::new(i as i32, j as i32));
        col.draw(ctx)?;
      }
    }

    for (_, row) in self.block.iter_mut().enumerate() {
      row.draw(ctx)?;
    }

    for (_, row) in self.place.iter_mut().enumerate() {
      row.draw(ctx)?;
    }

    for (_, row) in self.gbox.iter_mut().enumerate() {
      row.draw(ctx)?;
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

  fn request_move(&mut self, direction: Vector2<i32>) -> Option<Vector2<i32>> {
    let cell_start = self.player.get_position();
    let cell_start_type = self.get_cell_type(cell_start)?;
    let cell_dest = cell_start + direction;
    let cell_dest_type = self.get_cell_type(cell_dest)?;

    println!("start {:?} dest {:?}", cell_dest, cell_start);
    println!("cell_start_type {:?}", cell_dest_type);

    if cell_dest.x < 0 || cell_dest.x >= (10) {
      return None;
    }

    if cell_dest.y < 0 || cell_dest.y >= (10) {
      return None;
    }

    match cell_dest_type {
      PawnType::Ground => Some(cell_dest),
      PawnType::GBox(n) => {
        if let Some(pawn_to_push) = self.get_cell_type(cell_dest) {
          let dest = self.request_push_box(n, direction)?;
          let gbox = &mut self.gbox[n as usize];
          gbox.set_position(gbox.get_position() + dest);
          self.set_cell_type(cell_dest + direction, PawnType::GBox(n));
          self.set_cell_type(cell_start + direction, PawnType::Ground);
          Some(cell_dest)
        } else {
          Some(cell_dest)
        }
      }
      _ => None,
    }
  }

  fn request_push_box(&mut self, box_id: u8, direction: Vector2<i32>) -> Option<Vector2<i32>> {
    let gbox = &mut self.gbox[box_id as usize];
    let cell_start = gbox.get_position();
    let cell_start_type = self.get_cell_type(cell_start)?;
    let cell_dest = cell_start + direction;
    let cell_dest_type = self.get_cell_type(cell_dest)?;

    if cell_dest.x < 0 || cell_dest.x >= (10) {
      return None;
    }

    if cell_dest.y < 0 || cell_dest.y >= (10) {
      return None;
    }

    match cell_dest_type {
      PawnType::Ground => Some(direction),
      PawnType::Place => Some(direction),
      _ => None,
    }
  }

  fn get_cell_type(&self, position: Vector2<i32>) -> Option<PawnType> {
    for (i, row) in self.grid.iter().enumerate() {
      for (j, col) in row.iter().enumerate() {
        if position.x == i as i32 && position.y == j as i32 {
          return Some(*col);
        }
      }
    }

    None
  }

  fn set_cell_type(&mut self, position: Vector2<i32>, pawn_type: PawnType) {
    for (i, row) in self.grid.iter_mut().enumerate() {
      for (j, col) in row.iter_mut().enumerate() {
        if position.x == i as i32 && position.y == j as i32 {
          *col = pawn_type;
        }
      }
    }
  }
}
