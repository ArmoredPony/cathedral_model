use std::fmt::Display;

use ndarray::Array2;

pub mod board;
pub mod pieces;

// #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
// pub struct UVec2 {
//   x: usize,
//   y: usize,
// }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Team::White => "░░",
      Team::Black => "██",
      Team::None => "╳╳",
    })
  }
}
