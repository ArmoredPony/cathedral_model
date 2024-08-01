use std::fmt::Display;

use ndarray::{s, Array, Array2, ArrayView2, ArrayViewMut2};
use thiserror::Error;

use crate::{
  piece::{Piece, Placed, Released},
  Team,
};

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

  /// Checks if piece can be placed on board at given position. Returns possible
  /// error that can occur during placement.
  pub fn can_place_piece(
    &self,
    piece: &Piece<Released>,
    position: (usize, usize),
  ) -> Result<(), BoardError> {
    let tiles = self.get_interactive_tiles();
    for (x, y) in piece.occupied_coords_iter() {
      let tile = tiles
        .get((position.0 + x, position.1 + y))
        .ok_or(BoardError::PieceOutOfBounds)?;
      match tile {
        Tile::Wall => return Err(BoardError::PieceOutOfBounds),
        Tile::Empty(team) if piece.team.is_opposing_team(team) => {
          return Err(BoardError::PieceOnEnemyTile)
        }
        Tile::Occupied(_) => return Err(BoardError::PieceOnOccupiedTile),
        _ => (),
      }
    }
    Ok(())
  }

  /// Tries to put piece on board at given position. Returns same piece in
  /// `Placed` state or an error that occured.
  pub fn try_place_piece(
    &mut self,
    piece: Piece<Released>,
    position: (usize, usize),
  ) -> Result<Piece<Placed>, BoardError> {
    self.can_place_piece(&piece, position)?;
    let mut tiles = self.get_interactive_tiles_mut();
    for (x, y) in piece.occupied_coords_iter() {
      tiles[(position.0 + x, position.1 + y)] = Tile::Occupied(piece.team);
    }
    Ok(piece.placed_at(position))
  }

  /// Tries to put piece on board at given position. Panics if it can't.
  /// Returns sane piece in `Placed` state.
  pub fn place_piece(
    &mut self,
    piece: Piece<Released>,
    position: (usize, usize),
  ) -> Piece<Placed> {
    self
      .try_place_piece(piece, position)
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
    let tavern = Piece::new_tavern(Team::White);
    assert!(board.can_place_piece(&tavern, (0, 0)).is_ok());

    assert!(board.can_place_piece(&tavern, (9, 9)).is_ok());

    assert_eq!(
      board.can_place_piece(&tavern, (10, 0)),
      Err(BoardError::PieceOutOfBounds)
    );

    assert_eq!(
      board.can_place_piece(&tavern, (0, 10)),
      Err(BoardError::PieceOutOfBounds)
    );

    let cathedral = Piece::new_cathedral();
    assert!(board.can_place_piece(&cathedral, (0, 0)).is_ok());

    assert!(board.can_place_piece(&cathedral, (6, 7)).is_ok());

    assert_eq!(
      board.can_place_piece(&cathedral, (7, 6)),
      Err(BoardError::PieceOutOfBounds)
    );

    assert_eq!(
      board.can_place_piece(&cathedral, (6, 8)),
      Err(BoardError::PieceOutOfBounds)
    );
  }

  #[test]
  fn try_place_piece() {
    let mut board = Board::default();
    println!("{board}");

    let tavern = Piece::new_tavern(Team::White);
    let tavern = board.place_piece(tavern, (9, 0));
    assert_eq!(tavern.position(), (9, 0));

    // TODO: finish tests
  }
}
