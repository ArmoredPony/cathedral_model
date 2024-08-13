use std::{fmt::Display, ops::Add};

use ndarray::Array2;

pub mod board;
pub mod error;
pub mod piece;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Position {
  x: usize,
  y: usize,
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Team {
  White,
  Black,
  None,
}

impl Team {
  pub fn is_opposing_team(&self, team: &Team) -> bool {
    matches!(
      (*self, *team),
      (Team::White, Team::Black) | (Team::Black, Team::White)
    )
  }
}

impl Display for Team {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      Team::White => "░░",
      Team::Black => "██",
      Team::None => "╳╳",
    })
  }
}
