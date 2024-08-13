use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

impl From<(usize, usize)> for Position {
  fn from(value: (usize, usize)) -> Self {
    Position {
      x: value.0,
      y: value.1,
    }
  }
}

impl Add for Position {
  type Output = Position;

  fn add(self, rhs: Self) -> Self::Output {
    Position {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}
