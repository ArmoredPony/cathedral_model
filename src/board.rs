use std::{collections::HashMap, fmt::Display};

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
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Tile::Empty(team) if *team == Team::None => write!(f, "  "),
      Tile::Empty(team) | Tile::Occupied(team) => write!(f, "{team}"),
      Tile::Wall => write!(f, "╲╲"),
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
  #[error("place doesn't belong to this board")]
  PieceNotOnBoard,
}

pub struct Board {
  tiles: Array2<Tile>,
  pieces: HashMap<Piece<Placed>, (usize, usize)>,
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
    Board {
      tiles,
      pieces: HashMap::new(),
    }
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
    let piece = piece.placed_at(position);
    self.pieces.insert(piece.clone(), position);
    Ok(piece)
  }

  /// Tries to put piece on board at given position. Panics if it can't.
  /// Returns same piece in `Placed` state.
  pub fn place_piece(
    &mut self,
    piece: Piece<Released>,
    position: (usize, usize),
  ) -> Piece<Placed> {
    self
      .try_place_piece(piece, position)
      .unwrap_or_else(|e| panic!("could not put piece on the board: {e}"))
  }

  /// Tries to remove piece from board. Returns same piece in `Released` state
  /// or an error that occured.
  pub fn try_remove_piece(
    &mut self,
    piece: Piece<Placed>,
  ) -> Result<Piece<Released>, BoardError> {
    let mut tiles = self.get_interactive_tiles_mut();
    let position = piece.position();
    for (x, y) in piece.occupied_coords_iter() {
      tiles[(position.1 + x, position.0 + y)] = Tile::Empty(Team::None);
    }
    self
      .pieces
      .remove(&piece)
      .ok_or(BoardError::PieceNotOnBoard)?;
    Ok(piece.released())
  }

  /// Tries to remove piece from board. Panics if it can't. Returns same piece
  /// in `Released` state.
  pub fn remove_piece(&mut self, piece: Piece<Placed>) -> Piece<Released> {
    self
      .try_remove_piece(piece)
      .unwrap_or_else(|_| panic!("could not remove piece from the board"))
  }
}

impl Default for Board {
  fn default() -> Self {
    Self::with_size(10)
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "\n   ")?;
    for i in 0..self.tiles.ncols() - 2 {
      write!(f, "{i:>2}")?;
    }
    writeln!(f)?;
    self
      .tiles
      .rows()
      .into_iter()
      .enumerate()
      .skip(1)
      .try_for_each(|(i, row)| {
        if (1..self.tiles.nrows() - 1).contains(&i) {
          write!(f, "{:>2} ", i - 1)?;
        } else {
          write!(f, "   ")?;
        }
        row
          .iter()
          .skip(1)
          .try_for_each(|cell| write!(f, "{cell}"))?;
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
  /// Then removes each placed piece from the board and check if the board is
  /// empty.
  #[test]
  fn test_fill_and_free_board() -> Result<(), BoardError> {
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
    let w_tavern1_placed = board.try_place_piece(w_tavern1, (0, 0))?;
    let w_abbey_placed = board.try_place_piece(w_abbey, (0, 0))?;
    let w_stable1_placed = board.try_place_piece(w_stable1, (3, 0))?;
    w_stable2.rotate_clockwise();
    let w_stable2_placed = board.try_place_piece(w_stable2, (4, 0))?;
    w_academy.rotate_clockwise();
    let w_academy_placed = board.try_place_piece(w_academy, (5, 0))?;
    let w_square_placed = board.try_place_piece(w_square, (7, 0))?;
    let w_tavern2_placed = board.try_place_piece(w_tavern2, (0, 2))?;
    w_manor.rotate_clockwise();
    w_manor.rotate_clockwise();
    let w_manor_placed = board.try_place_piece(w_manor, (1, 1))?;
    w_tower.rotate_counterclockwise();
    let w_tower_placed = board.try_place_piece(w_tower, (4, 1))?;
    let w_inn1_placed = board.try_place_piece(w_inn1, (0, 3))?;
    let w_infirmary_placed = board.try_place_piece(w_infirmary, (1, 3))?;
    w_castle.rotate_clockwise();
    let w_castle_placed = board.try_place_piece(w_castle, (3, 3))?;
    let w_bridge_placed = board.try_place_piece(w_bridge, (0, 5))?;
    w_inn2.rotate_counterclockwise();
    let w_inn2_placed = board.try_place_piece(w_inn2, (1, 5))?;

    let b_bridge_placed = board.try_place_piece(b_bridge, (9, 0))?;
    let b_tavern1_placed = board.try_place_piece(b_tavern1, (8, 2))?;
    b_manor.rotate_clockwise();
    b_manor.rotate_clockwise();
    let b_manor_placed = board.try_place_piece(b_manor, (6, 3))?;
    b_castle.rotate_clockwise();
    let b_castle_placed = board.try_place_piece(b_castle, (8, 3))?;
    b_inn1.rotate_counterclockwise();
    let b_inn1_placed = board.try_place_piece(b_inn1, (5, 4))?;
    let b_infirmary_placed = board.try_place_piece(b_infirmary, (2, 6))?;
    b_tower.rotate_clockwise();
    let b_tower_placed = board.try_place_piece(b_tower, (4, 6))?;
    b_abbey.rotate_clockwise();
    let b_abbey_placed = board.try_place_piece(b_abbey, (8, 6))?;
    b_academy.rotate_clockwise();
    b_academy.rotate_clockwise();
    let b_academy_placed = board.try_place_piece(b_academy, (0, 7))?;
    let b_square_placed = board.try_place_piece(b_square, (4, 8))?;
    b_stable1.rotate_clockwise();
    let b_stable1_placed = board.try_place_piece(b_stable1, (0, 9))?;
    let b_tavern2_placed = board.try_place_piece(b_tavern2, (3, 9))?;
    b_stable2.rotate_clockwise();
    let b_stable2_placed = board.try_place_piece(b_stable2, (6, 9))?;
    b_inn2.rotate_clockwise();
    b_inn2.rotate_clockwise();
    let b_inn2_placed = board.try_place_piece(b_inn2, (8, 8))?;

    let cathedral_placed = board.try_place_piece(cathedral, (6, 5))?;

    board.try_remove_piece(w_tavern1_placed)?;
    board.try_remove_piece(w_abbey_placed)?;
    board.try_remove_piece(w_stable1_placed)?;
    board.try_remove_piece(w_stable2_placed)?;
    board.try_remove_piece(w_academy_placed)?;
    board.try_remove_piece(w_square_placed)?;
    board.try_remove_piece(w_tavern2_placed)?;
    board.try_remove_piece(w_manor_placed)?;
    board.try_remove_piece(w_tower_placed)?;
    board.try_remove_piece(w_inn1_placed)?;
    board.try_remove_piece(w_infirmary_placed)?;
    board.try_remove_piece(w_castle_placed)?;
    board.try_remove_piece(w_bridge_placed)?;
    board.try_remove_piece(w_inn2_placed)?;
    board.try_remove_piece(b_bridge_placed)?;
    board.try_remove_piece(b_tavern1_placed)?;
    board.try_remove_piece(b_manor_placed)?;
    board.try_remove_piece(b_castle_placed)?;
    board.try_remove_piece(b_inn1_placed)?;
    board.try_remove_piece(b_infirmary_placed)?;
    board.try_remove_piece(b_tower_placed)?;
    board.try_remove_piece(b_abbey_placed)?;
    board.try_remove_piece(b_academy_placed)?;
    board.try_remove_piece(b_square_placed)?;
    board.try_remove_piece(b_stable1_placed)?;
    board.try_remove_piece(b_tavern2_placed)?;
    board.try_remove_piece(b_stable2_placed)?;
    board.try_remove_piece(b_inn2_placed)?;
    board.try_remove_piece(cathedral_placed)?;

    assert!(board
      .get_interactive_tiles()
      .iter()
      .all(|t| *t == Tile::Empty(Team::None)));

    Ok(())
  }
}
