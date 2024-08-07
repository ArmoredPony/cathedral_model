use std::{
  collections::{HashMap, HashSet},
  fmt::Display,
};

use ndarray::{s, Array, Array2, ArrayView2, ArrayViewMut2};

use crate::{
  error::BoardError,
  piece::{Piece, Placed, Released},
  Position,
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

pub struct Board {
  tiles: Array2<Tile>,
  pieces: HashMap<Position, Piece<Placed>>,
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
    position: Position,
  ) -> Result<(), BoardError> {
    let tiles = self.get_interactive_tiles();
    for Position { x, y } in piece.occupied_coords_iter() {
      let tile = tiles
        .get((position.y + x, position.x + y))
        .ok_or(BoardError::PieceOutOfBounds)?;
      match tile {
        Tile::Wall => return Err(BoardError::PieceOutOfBounds),
        Tile::Empty(team) if piece.team().is_opposing_team(team) => {
          return Err(BoardError::PieceOnEnemyTile)
        }
        Tile::Occupied(_) => return Err(BoardError::PieceOnOccupiedTile),
        _ => (),
      }
    }
    Ok(())
  }

  /// Tries to put piece on board at given position.
  pub fn try_place_piece_at(
    &mut self,
    piece: Piece<Released>,
    position: Position,
  ) -> Result<(), BoardError> {
    self.can_place_piece(&piece, position)?;
    let piece = piece.placed_at(position);
    let mut tiles = self.get_interactive_tiles_mut();
    for Position { x, y } in piece.occupied_coords_iter() {
      tiles[(position.y + x, position.x + y)] = Tile::Occupied(piece.team());
    }
    let first_occupied_position = piece
      .occupied_coords_iter()
      .next()
      .expect("piece must occupy at least one tile");
    self
      .pieces
      .insert(position + first_occupied_position, piece);
    Ok(())
  }

  /// Tries to put piece on board at given position. Panics if it can't.
  pub fn place_piece_at(&mut self, piece: Piece<Released>, position: Position) {
    self
      .try_place_piece_at(piece, position)
      .unwrap_or_else(|e| panic!("could not put piece on the board: {e}"))
  }

  /// Tries to remove piece from board.
  /// Returns that piece in `Released` state or an error that occured.
  pub fn try_remove_piece(
    &mut self,
    piece: Piece<Placed>,
    found_at: Position,
  ) -> Result<Piece<Released>, BoardError> {
    match self.pieces.remove(&found_at) {
      Some(_) => (),
      None => return Err(BoardError::PieceNotOnBoard),
    }
    let mut tiles = self.get_interactive_tiles_mut();
    for Position { x, y } in piece.occupied_coords_iter() {
      tiles[(piece.position().y + x, piece.position().x + y)] =
        Tile::Empty(Team::None);
    }
    Ok(piece.released())
  }

  /// Tries to remove piece from board.
  /// Panics if it can't. Returns removed piece in `Released` state.
  pub fn remove_piece(
    &mut self,
    piece: Piece<Placed>,
    found_at: Position,
  ) -> Piece<Released> {
    self
      .try_remove_piece(piece, found_at)
      .unwrap_or_else(|e| panic!("{}", e))
  }

  /// Returns adjacent positions (vertically, horizontally and diagonally) to
  /// given position.
  fn adjacent_positions(position: Position) -> [Position; 8] {
    let Position { x, y } = position;
    [
      Position { x, y: y + 1 },
      Position { x: x + 1, y: y + 1 },
      Position { x: x + 1, y },
      Position { x: x + 1, y: y - 1 },
      Position { x, y: y - 1 },
      Position { x: x - 1, y: y - 1 },
      Position { x: x - 1, y },
      Position { x: x - 1, y: y + 1 },
    ]
  }

  /// Returns true if a tile is traversible by flood fill algorithm.
  fn is_traversible(&self, p: Position, team: Team) -> bool {
    match self.tiles.get((p.x, p.y)).expect("went out of bounds") {
      Tile::Occupied(t) if *t != team => true,
      Tile::Empty(_) => true,
      _ => false,
    }
  }

  /// Searches occupiable tile positions using flood fill algorithm and puts
  /// them into set.
  fn flood_fill_positions_into_set(
    &self,
    position: Position,
    team: Team,
    set: &mut HashSet<Position>,
  ) {
    if set.contains(&position) {
      return;
    }
    set.insert(position);
    let adjacent_positions = Self::adjacent_positions(position);
    set.extend(adjacent_positions.to_owned());
    for p in adjacent_positions
      .into_iter()
      .filter(|p| self.is_traversible(*p, team))
    {
      self.flood_fill_positions_into_set(p, team, set);
    }
  }

  /// Searches for groups of tiles that can be claimed by placed piece.
  fn tile_groups(&self, piece: &Piece<Placed>) -> Vec<HashSet<Position>> {
    let initial_tiles_positions: HashSet<Position> = piece
      .occupied_coords_iter()
      .flat_map(Self::adjacent_positions)
      .collect::<HashSet<_>>()
      .into_iter()
      .filter(|p| self.is_traversible(*p, piece.team()))
      .collect();
    let mut groups: Vec<HashSet<Position>> = Vec::new();
    for p in initial_tiles_positions {
      if groups.iter().any(|set| set.contains(&p)) {
        continue;
      }
      let mut set: HashSet<Position> = HashSet::new();
      self.flood_fill_positions_into_set(p, piece.team(), &mut set);
      groups.push(set);
    }
    groups
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
  fn test_can_place_piece() -> Result<(), BoardError> {
    let board = Board::default();
    let tavern = Piece::new_tavern(Team::White);
    board.can_place_piece(&tavern, (0, 0).into())?;

    board.can_place_piece(&tavern, (9, 9).into())?;

    assert_eq!(
      board.can_place_piece(&tavern, (10, 0).into()),
      Err(BoardError::PieceOutOfBounds)
    );

    assert_eq!(
      board.can_place_piece(&tavern, (0, 10).into()),
      Err(BoardError::PieceOutOfBounds)
    );

    let cathedral = Piece::new_cathedral();
    board.can_place_piece(&cathedral, (0, 0).into())?;

    board.can_place_piece(&cathedral, (7, 6).into())?;

    assert_eq!(
      board.can_place_piece(&cathedral, (7, 7).into()),
      Err(BoardError::PieceOutOfBounds)
    );

    assert_eq!(
      board.can_place_piece(&cathedral, (8, 6).into()),
      Err(BoardError::PieceOutOfBounds)
    );

    Ok(())
  }

  #[test]
  fn test_try_place_piece() -> Result<(), BoardError> {
    let mut board = Board::default();

    let inn = Piece::new_inn(Team::White);
    assert_eq!(
      board
        .try_place_piece_at(inn, (0, 9).into())
        .expect_err("must be error"),
      BoardError::PieceOutOfBounds
    );

    let white_tavern = Piece::new_tavern(Team::White);
    board.try_place_piece_at(white_tavern, (1, 2).into())?;

    let black_tavern = Piece::new_tavern(Team::Black);
    assert_eq!(
      board
        .try_place_piece_at(black_tavern, (1, 2).into())
        .expect_err("must be error"),
      BoardError::PieceOnOccupiedTile
    );

    let black_infirmary = Piece::new_infirmary(Team::Black);
    assert_eq!(
      board
        .try_place_piece_at(black_infirmary, (1, 1).into())
        .expect_err("must be error"),
      BoardError::PieceOnOccupiedTile
    );

    Ok(())
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

    board.try_place_piece_at(w_tavern1, (0, 0).into())?;
    board.try_place_piece_at(w_abbey, (0, 0).into())?;
    board.try_place_piece_at(w_stable1, (3, 0).into())?;
    w_stable2.rotate_clockwise();
    board.try_place_piece_at(w_stable2, (4, 0).into())?;
    w_academy.rotate_clockwise();
    board.try_place_piece_at(w_academy, (5, 0).into())?;
    board.try_place_piece_at(w_square, (7, 0).into())?;
    board.try_place_piece_at(w_tavern2, (0, 2).into())?;
    w_manor.rotate_clockwise();
    w_manor.rotate_clockwise();
    board.try_place_piece_at(w_manor, (1, 1).into())?;
    w_tower.rotate_counterclockwise();
    board.try_place_piece_at(w_tower, (4, 1).into())?;
    board.try_place_piece_at(w_inn1, (0, 3).into())?;
    board.try_place_piece_at(w_infirmary, (1, 3).into())?;
    w_castle.rotate_clockwise();
    board.try_place_piece_at(w_castle, (3, 3).into())?;
    board.try_place_piece_at(w_bridge, (0, 5).into())?;
    w_inn2.rotate_counterclockwise();
    board.try_place_piece_at(w_inn2, (1, 5).into())?;

    board.try_place_piece_at(b_bridge, (9, 0).into())?;
    board.try_place_piece_at(b_tavern1, (8, 2).into())?;
    b_manor.rotate_clockwise();
    b_manor.rotate_clockwise();
    board.try_place_piece_at(b_manor, (6, 3).into())?;
    b_castle.rotate_clockwise();
    board.try_place_piece_at(b_castle, (8, 3).into())?;
    b_inn1.rotate_counterclockwise();
    board.try_place_piece_at(b_inn1, (5, 4).into())?;
    board.try_place_piece_at(b_infirmary, (2, 6).into())?;
    b_tower.rotate_clockwise();
    board.try_place_piece_at(b_tower, (4, 6).into())?;
    b_abbey.rotate_clockwise();
    board.try_place_piece_at(b_abbey, (8, 6).into())?;
    b_academy.rotate_clockwise();
    b_academy.rotate_clockwise();
    board.try_place_piece_at(b_academy, (0, 7).into())?;
    board.try_place_piece_at(b_square, (4, 8).into())?;
    b_stable1.rotate_clockwise();
    board.try_place_piece_at(b_stable1, (0, 9).into())?;
    board.try_place_piece_at(b_tavern2, (3, 9).into())?;
    b_stable2.rotate_clockwise();
    board.try_place_piece_at(b_stable2, (6, 9).into())?;
    b_inn2.rotate_clockwise();
    b_inn2.rotate_clockwise();
    board.try_place_piece_at(b_inn2, (8, 8).into())?;

    board.try_place_piece_at(cathedral, (6, 5).into())?;

    println!("{board}");
    assert!(board
      .get_interactive_tiles()
      .iter()
      .all(|t| matches!(t, Tile::Occupied(_))));

    let piece_pos = board.pieces.clone().into_iter().collect::<Vec<_>>();
    for (pos, piece) in piece_pos.into_iter() {
      board.try_remove_piece(piece, pos)?;
    }
    println!("{board}");
    assert!(board
      .get_interactive_tiles()
      .iter()
      .all(|t| matches!(t, Tile::Empty(Team::None))));

    Ok(())
  }
}
