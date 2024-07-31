use std::fmt::Display;

use ndarray::{Array, Array2};

use crate::Team;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
  Empty(Team),
  Occupied(Team),
  Wall,
}

impl Display for Cell {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Cell::Empty(team) if *team == Team::None => write!(f, "  "),
      Cell::Empty(team) | Cell::Occupied(team) => write!(f, "{team}"),
      Cell::Wall => write!(f, "╱╱"),
    }
  }
}

pub struct Board {
  cells: Array2<Cell>,
}

impl Board {
  pub fn with_size(size: usize) -> Self {
    let mut cells = Array::from_elem(
      (size + 2, size + 2), //
      Cell::Empty(Team::None),
    );
    cells.row_mut(0).fill(Cell::Wall);
    cells.row_mut(size - 1).fill(Cell::Wall);
    cells.column_mut(0).fill(Cell::Wall);
    cells.column_mut(size - 1).fill(Cell::Wall);
    Board { cells }
  }
}

impl Default for Board {
  fn default() -> Self {
    Self::with_size(10)
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.cells.rows().into_iter().try_for_each(|row| {
      row.iter().try_for_each(|cell| write!(f, "{cell}"))?;
      writeln!(f)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_print() {
    println!("{}", Board::default());
  }
}
