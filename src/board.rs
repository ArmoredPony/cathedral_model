use std::fmt::Display;

use ndarray::{Array, Array2};
use thiserror::Error;

use crate::{piece::Piece, Team};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
  Empty(Team),
  Occupied(Team),
  Wall,
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Tile::Empty(team) if *team == Team::None => write!(f, "  "),
      Tile::Empty(team) | Tile::Occupied(team) => write!(f, "{team}"),
      Tile::Wall => write!(f, "╱╱"),
    }
  }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BoardError {
  #[error("piece was placed out of bounds")]
  PieceOutOfBounds,
  #[error("piece was placed on occupied tile")]
  PieceOnOccupiedTile,
  #[error("piece was placed on other team's tile")]
  PieceOnEnemyTile,
}

pub struct Board {
  tiles: Array2<Tile>,
}

impl Board {
  pub fn with_size(size: usize) -> Self {
    let mut tiles = Array::from_elem(
      (size + 2, size + 2), //
      Tile::Empty(Team::None),
    );
    tiles.row_mut(0).fill(Tile::Wall);
    tiles.row_mut(size + 1).fill(Tile::Wall);
    tiles.column_mut(0).fill(Tile::Wall);
    tiles.column_mut(size + 1).fill(Tile::Wall);
    Board { tiles }
  }

  /// Returns interactive tiles of the board.
  /// I.e. returns those tiles that a play can put a piece on.
  #[allow(clippy::reversed_empty_ranges)]
  pub fn get_interactive_tiles(&self) -> ArrayView2<'_, Tile> {
    self.tiles.slice(s![1..-1, 1..-1])
  }

  /// Returns interactive tiles of the board.
  /// I.e. returns those tiles that a play can put a piece on.
  #[allow(clippy::reversed_empty_ranges)]
  pub fn get_interactive_tiles_mut(&mut self) -> ArrayViewMut2<'_, Tile> {
    self.tiles.slice_mut(s![1..-1, 1..-1])
  }

  // TODO: pass position
  pub fn try_place_piece(&mut self, piece: &Piece) -> Result<(), BoardError> {
    // TODO: replace `position` in `piece` after placing via copying it
    self.can_place_piece(piece)?;
    for (x, y) in piece.occupied_coords_iter() {
      let board_y = x + 1;
      let board_x = y + 1;
      self.tiles[(board_x, board_y)] = Tile::Occupied(piece.team);
    }
    // TODO: return new `piece`
    Ok(())
  }

  // TODO: pass position
  pub fn place_piece(&mut self, piece: &Piece) {
    self
      .try_place_piece(piece)
      .unwrap_or_else(|e| panic!("could not put piece on the board: {e}"))
  }
}

impl Default for Board {
  fn default() -> Self {
    Self::with_size(10)
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\n     ")?;
    for i in 0..self.tiles.ncols() - 2 {
      write!(f, "{i:>2}")?;
    }
    writeln!(f)?;
    self
      .tiles
      .rows()
      .into_iter()
      .enumerate()
      .try_for_each(|(i, row)| {
        if (1..self.tiles.nrows() - 1).contains(&i) {
          write!(f, "{:>2} ", i - 1)?;
        } else {
          write!(f, "   ")?;
        }
        row.iter().try_for_each(|cell| write!(f, "{cell}"))?;
        writeln!(f)
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_interactive_tiles_empty_on_empty_board() {
    let board = Board::default();
    let interactive_tiles = board.get_interactive_tiles();
    assert!(interactive_tiles
      .iter()
      .all(|tile| *tile == Tile::Empty(Team::None)));
  }

  #[test]
  fn test_can_place_piece() {
    let board = Board::default();
    let mut tavern = Piece::new_tavern((0, 0), Team::White);
    assert!(board.can_place_piece(&tavern).is_ok());

    tavern.position = (9, 9);
    assert!(board.can_place_piece(&tavern).is_ok());

    tavern.position = (10, 0);
    assert_eq!(
      board.can_place_piece(&tavern),
      Err(BoardError::PieceOutOfBounds)
    );

    tavern.position = (0, 10);
    assert_eq!(
      board.can_place_piece(&tavern),
      Err(BoardError::PieceOutOfBounds)
    );

    let mut cathedral = Piece::new_cathedral((0, 0));
    assert!(board.can_place_piece(&cathedral).is_ok());

    cathedral.position = (6, 7);
    assert!(board.can_place_piece(&cathedral).is_ok());

    cathedral.position = (7, 6);
    assert_eq!(
      board.can_place_piece(&cathedral),
      Err(BoardError::PieceOutOfBounds)
    );

    cathedral.position = (6, 8);
    assert_eq!(
      board.can_place_piece(&cathedral),
      Err(BoardError::PieceOutOfBounds)
    );
  }

  #[test]
  fn try_place_piece() {
    let mut board = Board::default();
    println!("{board}");

    let mut tavern = Piece::new_tavern((9, 0), Team::White);
    board.place_piece(&tavern);
    println!("{board}");

    tavern.position = (0, 9);
    board.place_piece(&tavern);
    println!("{board}");

    // TODO: finish tests
  }
}
