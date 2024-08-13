use std::fmt::Display;

use ndarray::Array2;

pub mod board;
pub mod error;
pub mod piece;
pub mod position;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Team {
  White,
  Black,
  None,
}

impl Team {
  pub fn is_opposing_team(&self, team: &Self) -> bool {
    matches!(
      (*self, *team),
      (Self::White, Self::Black) | (Self::Black, Self::White)
    )
  }
}

impl Display for Team {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      Self::White => "░░",
      Self::Black => "██",
      Self::None => "╳╳",
    })
  }
}
