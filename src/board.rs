use cgmath::Vector2;
use ggez::event::KeyCode;
use ggez::{timer, Context, GameResult};

use super::pawn::{Pawn, PawnType};

const VECTOR_RIGHT: Vector2<f32> = Vector2::new(1., 0.);
const VECTOR_LEFT: Vector2<f32> = Vector2::new(-1., 0.);
const VECTOR_DOWN: Vector2<f32> = Vector2::new(0., 1.);
const VECTOR_UP: Vector2<f32> = Vector2::new(0., -1.);
const VECTOR_ZERO: Vector2<f32> = Vector2::new(0., 0.);

pub struct Board {
  player: Pawn,
  grounds: Vec<Vec<Pawn>>,
  boxes: Vec<Pawn>,
  blocks: Vec<Pawn>,
  places: Vec<Pawn>,
  grid: Vec<Vec<PawnType>>,
  finished: bool,
  seconds_after_finish: f32,
  pub winner: bool,
}

impl Board {
  pub fn new(ctx: &mut Context, grid_size: (usize, usize), cell_size: (usize, usize)) -> Self {
    let grid: Vec<Vec<PawnType>> = vec![vec![PawnType::Ground; grid_size.0]; grid_size.1];
    let grounds: Vec<Vec<Pawn>> = vec![
      vec![
        Pawn::new(
          ctx,
          "ground.png".to_string(),
          PawnType::Ground,
          Vector2::<f32>::new(0., 0.),
          cell_size
        );
        grid_size.0
      ];
      grid_size.1
    ];

    let player = Pawn::new(
      ctx,
      "player.png".to_string(),
      PawnType::Player,
      Vector2::<f32>::new(0., 0.),
      cell_size,
    );

    let places = vec![];
    let blocks = vec![];
    let boxes = vec![];

    Board {
      player,
      grounds,
      places,
      blocks,
      boxes,
      grid,
      seconds_after_finish: 0.,
      finished: false,
      winner: false,
    }
  }

  pub fn update(&mut self, ctx: &mut Context) {
    const DESIRED_FPS: u32 = 60;

    while timer::check_update_time(ctx, DESIRED_FPS) {
      let seconds = 1.0 / (DESIRED_FPS as f32);
      if self.finished {
        self.seconds_after_finish += seconds;
        if self.seconds_after_finish >= 0.5 {
          self.winner = true;
        }
      }
    }
  }

  pub fn set_player_start(&mut self, position: Vector2<f32>) {
    self.player.set_position(position);
  }

  pub fn add_block(
    &mut self,
    ctx: &mut Context,
    position: Vector2<f32>,
    cell_size: (usize, usize),
  ) {
    self.blocks.push(Pawn::new(
      ctx,
      "block.png".to_string(),
      PawnType::Block,
      position,
      cell_size,
    ));

    self.set_cell_type(position, PawnType::Block);
  }

  pub fn add_box(&mut self, ctx: &mut Context, position: Vector2<f32>, cell_size: (usize, usize)) {
    let boxes_len = self.boxes.len() as usize;
    self.boxes.push(Pawn::new(
      ctx,
      "box.png".to_string(),
      PawnType::GBox(boxes_len),
      position,
      cell_size,
    ));

    self.set_cell_type(position, PawnType::GBox(boxes_len));
  }

  pub fn add_place(
    &mut self,
    ctx: &mut Context,
    position: Vector2<f32>,
    cell_size: (usize, usize),
  ) {
    self.places.push(Pawn::new(
      ctx,
      "place.png".to_string(),
      PawnType::Place,
      position,
      cell_size,
    ));

    self.set_cell_type(position, PawnType::Place);
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
    for (i, row) in self.grounds.iter_mut().enumerate() {
      for (j, col) in row.iter_mut().enumerate() {
        col.set_position(Vector2::<f32>::new(i as f32, j as f32));
        col.draw(ctx)?;
      }
    }

    for (_, place) in self.places.iter_mut().enumerate() {
      place.draw(ctx)?;
    }

    for (_, block) in self.blocks.iter_mut().enumerate() {
      block.draw(ctx)?;
    }

    for (_, gbox) in self.boxes.iter_mut().enumerate() {
      gbox.draw(ctx)?;
    }

    self.player.draw(ctx)?;

    Ok(())
  }

  pub fn key_down(&mut self, keycode: KeyCode) {
    if self.finished {
      return;
    }

    let dest = match keycode {
      KeyCode::Right => VECTOR_RIGHT,
      KeyCode::Left => VECTOR_LEFT,
      KeyCode::Up => VECTOR_UP,
      KeyCode::Down => VECTOR_DOWN,
      _ => VECTOR_ZERO,
    };

    if let Some(cell_dest) = self.request_move(dest) {
      self.player.set_position(cell_dest);
      if self.check_winner() {
        self.finished = true;
      }
    }

    if keycode == KeyCode::I {
      for (i, row) in self.grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
          println!("Col {} Row {} Type {:?}", i, j, col);
        }
      }
    }
  }

  fn request_move(&mut self, direction: Vector2<f32>) -> Option<Vector2<f32>> {
    let cell_start = self.player.get_position();
    let _cell_start_type = self.get_cell_type(cell_start)?;
    let cell_dest = cell_start + direction;
    let cell_dest_type = self.get_cell_type(cell_dest)?;

    // println!("start {:?} dest {:?}", cell_dest, cell_start);
    // println!("cell_start_type {:?}", cell_dest_type);

    if cell_dest.x < 0. || cell_dest.x >= (10.) {
      return None;
    }

    if cell_dest.y < 0. || cell_dest.y >= (10.) {
      return None;
    }

    match cell_dest_type {
      PawnType::Ground | PawnType::Place => Some(cell_dest),
      PawnType::GBox(n) => {
        if let Some(_pawn_to_push) = self.get_cell_type(cell_dest) {
          // println!("Push box {}", n);
          let dest = self.request_push_box(n, direction)?;
          let gbox = &mut self.boxes[n as usize];
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

  fn request_push_box(&mut self, box_id: usize, direction: Vector2<f32>) -> Option<Vector2<f32>> {
    let boxes = &mut self.boxes[box_id as usize];
    let cell_start = boxes.get_position();
    let _cell_start_type = self.get_cell_type(cell_start)?;
    let cell_dest = cell_start + direction;
    let cell_dest_type = self.get_cell_type(cell_dest)?;

    if cell_dest.x < 0. || cell_dest.x >= (10.) {
      return None;
    }

    if cell_dest.y < 0. || cell_dest.y >= (10.) {
      return None;
    }

    match cell_dest_type {
      PawnType::Ground => Some(direction),
      PawnType::Place => Some(direction),
      _ => None,
    }
  }

  fn check_winner(&self) -> bool {
    for (_, row) in self.grid.iter().enumerate() {
      for (_, col) in row.iter().enumerate() {
        if *col == PawnType::Place {
          return false;
        }
      }
    }

    true
  }

  fn get_cell_type(&self, position: Vector2<f32>) -> Option<PawnType> {
    for (i, row) in self.grid.iter().enumerate() {
      for (j, col) in row.iter().enumerate() {
        if position.x == i as f32 && position.y == j as f32 {
          return Some(*col);
        }
      }
    }

    None
  }

  fn set_cell_type(&mut self, position: Vector2<f32>, pawn_type: PawnType) {
    for (i, row) in self.grid.iter_mut().enumerate() {
      for (j, col) in row.iter_mut().enumerate() {
        if position.x == i as f32 && position.y == j as f32 {
          *col = pawn_type;
        }
      }
    }
  }
}
