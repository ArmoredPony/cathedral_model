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
        .get((position.1 + x, position.0 + y))
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
      tiles[(position.1 + x, position.0 + y)] = Tile::Occupied(piece.team);
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

    assert!(board.can_place_piece(&cathedral, (7, 6)).is_ok());

    assert_eq!(
      board.can_place_piece(&cathedral, (7, 7)),
      Err(BoardError::PieceOutOfBounds)
    );

    assert_eq!(
      board.can_place_piece(&cathedral, (8, 6)),
      Err(BoardError::PieceOutOfBounds)
    );
  }

  #[test]
  fn test_try_place_piece() {
    let mut board = Board::default();

    let inn = Piece::new_inn(Team::White);
    assert_eq!(
      board
        .try_place_piece(inn, (0, 9))
        .expect_err("must be error"),
      BoardError::PieceOutOfBounds
    );

    let white_tavern = Piece::new_tavern(Team::White);
    let white_tavern = board.try_place_piece(white_tavern, (1, 2)).unwrap();
    assert_eq!(white_tavern.position(), (1, 2));

    let black_tavern = Piece::new_tavern(Team::Black);
    assert_eq!(
      board
        .try_place_piece(black_tavern, (1, 2))
        .expect_err("must be error"),
      BoardError::PieceOnOccupiedTile
    );

    let black_infirmary = Piece::new_infirmary(Team::Black);
    assert_eq!(
      board
        .try_place_piece(black_infirmary, (1, 1))
        .expect_err("must be error"),
      BoardError::PieceOnOccupiedTile
    );
  }

  /// Test if it is possible to fill the enitre board using all white pieces,
  /// black pieces and the cathedral. There should be no empty tiles left.
  #[test]
  fn test_fill_board_with_pieces() -> Result<(), BoardError> {
    let w_tavern1 = Piece::new_tavern(Team::White);
    let w_tavern2 = Piece::new_tavern(Team::White);
    let w_stable1 = Piece::new_stable(Team::White);
    let mut w_stable2 = Piece::new_stable(Team::White);
    let w_inn1 = Piece::new_inn(Team::White);
    let mut w_inn2 = Piece::new_inn(Team::White);
    let w_bridge = Piece::new_bridge(Team::White);
    let w_square = Piece::new_square(Team::White);
    let mut w_manor = Piece::new_manor(Team::White);
    let w_abbey = Piece::new_abbey(Team::White);
    let mut w_academy = Piece::new_academy(Team::White);
    let w_infirmary = Piece::new_infirmary(Team::White);
    let mut w_castle = Piece::new_castle(Team::White);
    let mut w_tower = Piece::new_tower(Team::White);

    let b_tavern1 = Piece::new_tavern(Team::Black);
    let b_tavern2 = Piece::new_tavern(Team::Black);
    let mut b_stable1 = Piece::new_stable(Team::Black);
    let mut b_stable2 = Piece::new_stable(Team::Black);
    let mut b_inn1 = Piece::new_inn(Team::Black);
    let mut b_inn2 = Piece::new_inn(Team::Black);
    let b_bridge = Piece::new_bridge(Team::Black);
    let b_square = Piece::new_square(Team::Black);
    let mut b_manor = Piece::new_manor(Team::Black);
    let mut b_abbey = Piece::new_abbey(Team::Black);
    let mut b_academy = Piece::new_academy(Team::Black);
    let b_infirmary = Piece::new_infirmary(Team::Black);
    let mut b_castle = Piece::new_castle(Team::Black);
    let mut b_tower = Piece::new_tower(Team::Black);

    let cathedral = Piece::new_cathedral();

    let mut board = Board::default();
    board.try_place_piece(w_tavern1, (0, 0))?;
    board.try_place_piece(w_abbey, (0, 0))?;
    board.try_place_piece(w_stable1, (3, 0))?;
    w_stable2.rotate_clockwise();
    board.try_place_piece(w_stable2, (4, 0))?;
    w_academy.rotate_clockwise();
    board.try_place_piece(w_academy, (5, 0))?;
    board.try_place_piece(w_square, (7, 0))?;
    board.try_place_piece(w_tavern2, (0, 2))?;
    w_manor.rotate_clockwise();
    w_manor.rotate_clockwise();
    board.try_place_piece(w_manor, (1, 1))?;
    w_tower.rotate_counterclockwise();
    board.try_place_piece(w_tower, (4, 1))?;
    board.try_place_piece(w_inn1, (0, 3))?;
    board.try_place_piece(w_infirmary, (1, 3))?;
    w_castle.rotate_clockwise();
    board.try_place_piece(w_castle, (3, 3))?;
    board.try_place_piece(w_bridge, (0, 5))?;
    w_inn2.rotate_counterclockwise();
    board.try_place_piece(w_inn2, (1, 5))?;

    board.try_place_piece(b_bridge, (9, 0))?;
    board.try_place_piece(b_tavern1, (8, 2))?;
    b_manor.rotate_clockwise();
    b_manor.rotate_clockwise();
    board.try_place_piece(b_manor, (6, 3))?;
    b_castle.rotate_clockwise();
    board.try_place_piece(b_castle, (8, 3))?;
    b_inn1.rotate_counterclockwise();
    board.try_place_piece(b_inn1, (5, 4))?;
    board.try_place_piece(b_infirmary, (2, 6))?;
    b_tower.rotate_clockwise();
    board.try_place_piece(b_tower, (4, 6))?;
    b_abbey.rotate_clockwise();
    board.try_place_piece(b_abbey, (8, 6))?;
    b_academy.rotate_clockwise();
    b_academy.rotate_clockwise();
    board.try_place_piece(b_academy, (0, 7))?;
    board.try_place_piece(b_square, (4, 8))?;
    b_stable1.rotate_clockwise();
    board.try_place_piece(b_stable1, (0, 9))?;
    board.try_place_piece(b_tavern2, (3, 9))?;
    b_stable2.rotate_clockwise();
    board.try_place_piece(b_stable2, (6, 9))?;
    b_inn2.rotate_clockwise();
    b_inn2.rotate_clockwise();
    board.try_place_piece(b_inn2, (8, 8))?;

    board.try_place_piece(cathedral, (6, 5))?;

    Ok(())
  }
}
