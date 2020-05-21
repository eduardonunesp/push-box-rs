// Player
pub const A: i32 = 9;

// Ground
pub const G: i32 = 0;

// Block
pub const B: i32 = 1;

// Box
pub const X: i32 = 2;

// Place
pub const P: i32 = 3;

// LEVEL 0
pub const LEVEL_0: &'static [&'static [i32]] = &[
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, A, G, G, G, X, G, G, P, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
];

// LEVEL 1
pub const LEVEL_1: &'static [&'static [i32]] = &[
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, G, X, P, B, B, B],
  &[B, B, B, B, G, B, B, B, B, B],
  &[B, A, G, G, G, X, G, G, P, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
];

// LEVEL 2
pub const LEVEL_2: &'static [&'static [i32]] = &[
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, P, B, B, B, B, B, B],
  &[B, B, B, G, G, X, P, B, B, B],
  &[B, B, B, X, G, B, B, B, B, B],
  &[B, A, G, G, G, X, G, G, P, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
  &[B, B, B, B, B, B, B, B, B, B],
];

pub const LEVELS: &'static [&'static [&'static [i32]]] = &[LEVEL_0, LEVEL_1, LEVEL_2];
