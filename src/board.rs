use std::{
  collections::{HashMap, HashSet},
  fmt::Display,
};

use ndarray::{Array, Array2};

use crate::{
  error::BoardError,
  piece::{Piece, Placed, Released},
  position::Position,
  Team,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
  Empty(Team),
  Occupied(Team),
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Tile::Empty(team) if *team == Team::None => write!(f, "  "),
      Tile::Empty(team) | Tile::Occupied(team) => write!(f, "{team}"),
    }
  }
}

pub struct Board {
  tiles: Array2<Tile>,
  pieces: HashMap<Position, Piece<Placed>>,
}

impl Board {
  pub fn with_size(size: usize) -> Self {
    Self {
      tiles: Array::from_elem((size, size), Tile::Empty(Team::None)),
      pieces: HashMap::new(),
    }
  }

  /// Returns board's size as position.
  pub fn size(&self) -> Position {
    Position::from(self.tiles.dim())
  }

  /// Checks if piece can be placed on board at given position. Returns possible
  /// error that can occur during placement.
  pub fn can_place_piece(
    &self,
    piece: &Piece<Released>,
    position: Position,
  ) -> Result<(), BoardError> {
    for p in piece.occupied_positions_iter(position) {
      let tile = self
        .tiles
        .get((p.x, p.y))
        .ok_or(BoardError::PieceOutOfBounds(p))?;
      match tile {
        Tile::Empty(team) if piece.team().is_opposing_team(team) => {
          return Err(BoardError::PieceOnEnemyTile(p))
        }
        Tile::Occupied(_) => return Err(BoardError::PieceOnOccupiedTile(p)),
        _ => (),
      }
    }
    Ok(())
  }

  /// Tries to put piece on board at given position.
  pub fn try_place_piece(
    &mut self,
    piece: Piece<Released>,
    position: Position,
  ) -> Result<Vec<Piece<Released>>, BoardError> {
    self.can_place_piece(&piece, position)?;
    let piece = piece.placed_at(position);

    let removed_pieces = Vec::<Piece<Released>>::new();

    for p in piece.occupied_positions_iter() {
      self.tiles[(p.x, p.y)] = Tile::Occupied(piece.team());
    }

    let first_occupied_position = piece
      .occupied_positions_iter()
      .next()
      .expect("piece must occupy at least one tile");
    self.pieces.insert(first_occupied_position, piece);

    Ok(removed_pieces)
  }

  /// Tries to put piece on board at given position. Panics if it can't.
  pub fn place_piece(
    &mut self,
    piece: Piece<Released>,
    position: Position,
  ) -> Vec<Piece<Released>> {
    self
      .try_place_piece(piece, position)
      .unwrap_or_else(|e| panic!("could not put piece on the board: {e}"))
  }

  /// Tries to remove piece from board.
  /// Returns removed piece in `Released` state or an error that occured.
  pub fn try_remove_piece(
    &mut self,
    position: Position,
  ) -> Result<Piece<Released>, BoardError> {
    let piece = match self.pieces.remove(&position) {
      Some(piece) => piece,
      None => return Err(BoardError::PieceNotOnBoard),
    };
    for p in piece.occupied_positions_iter() {
      self.tiles[(p.x, p.y)] = Tile::Empty(Team::None);
    }
    Ok(piece.released())
  }

  /// Tries to remove piece from board.
  /// Panics if it can't. Returns removed piece in `Released` state.
  pub fn remove_piece(&mut self, position: Position) -> Piece<Released> {
    self
      .try_remove_piece(position)
      .unwrap_or_else(|e| panic!("{}", e))
  }

  /// Returns `true` if `position` neighbours a wall tile position.
  fn near_wall(&self, position: Position) -> bool {
    let max_position = self.size();
    position.x == 0
      || position.y == 0
      || position.x == max_position.x
      || position.y == max_position.y
  }

  /// Returns `true` if tile at given `position` may form enclosing border
  /// on this `team`'s turn.
  fn does_position_form_border(&self, position: Position, team: Team) -> bool {
    matches!(
      self.tiles[(position.x, position.y)],
      Tile::Occupied(t) if t == team
    )
  }

  /// Returns `true` if tile at given `position` can be captured by playing a
  /// piece of `team`.
  fn is_position_capturable(&self, position: Position, team: Team) -> bool {
    matches!(
      self.tiles[(position.x, position.y)],
      Tile::Empty(t) | Tile::Occupied(t) if t != team
    )
  }

  /// Returns a set of unique capturable positions adjacent to given `piece`.
  fn adjacent_capturable_positions_for_piece(
    &self,
    piece: &Piece<Placed>,
  ) -> HashSet<Position> {
    piece
      .occupied_positions_iter()
      .flat_map(|p| p.diagonal_adjacent_positions_iter(self.size()))
      .filter(|p| self.is_position_capturable(*p, piece.team()))
      .collect()
  }

  /// Finds and returns a set of tiles in the same group with tile with
  /// `initial_position`.
  fn find_tile_set(
    &self,
    initial_position: Position,
    team: Team,
  ) -> HashSet<Position> {
    fn flood_fill_into_set(
      board: &Board,
      position: Position,
      team: Team,
      set: &mut HashSet<Position>,
    ) {
      if set.contains(&position) {
        return;
      }
      set.insert(position);
      position
        .diagonal_adjacent_positions_iter(board.size())
        .filter(|p| board.is_position_capturable(*p, team))
        .for_each(|p| flood_fill_into_set(board, p, team, set));
    }

    let mut set = HashSet::new();
    flood_fill_into_set(self, initial_position, team, &mut set);
    set
  }

  /// Returns sets of capturable tiles' positions.
  fn find_tile_sets(&self, piece: &Piece<Placed>) -> Vec<HashSet<Position>> {
    let mut groups: Vec<HashSet<Position>> = Vec::new();
    let initial_tiles_positions: HashSet<Position> = piece
      .occupied_positions_iter()
      .flat_map(|p| p.diagonal_adjacent_positions_iter(self.size()))
      .filter(|p| self.is_position_capturable(*p, piece.team()))
      .inspect(|p| println!("{p}"))
      .collect::<HashSet<_>>();
    for p in initial_tiles_positions {
      if !groups.iter().any(|set| set.contains(&p)) {
        let set = self.find_tile_set(p, piece.team());
        groups.push(set);
      }
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
    for i in 0..self.tiles.ncols() {
      write!(f, "{i:>2}")?;
    }
    writeln!(f)?;
    self
      .tiles
      .rows()
      .into_iter()
      .enumerate()
      .try_for_each(|(i, row)| {
        if (0..self.tiles.nrows()).contains(&i) {
          write!(f, "{:>2} ", i)?;
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
  fn test_max_position() {
    let board = Board::with_size(10);
    assert_eq!(board.size(), (10, 10).into());

    let board = Board::with_size(5);
    assert_eq!(board.size(), (5, 5).into());
  }

  #[test]
  fn test_interactive_tiles_empty_on_empty_board() {
    let board = Board::default();
    assert!(board
      .tiles
      .iter()
      .all(|tile| *tile == Tile::Empty(Team::None)));
  }

  #[test]
  fn test_can_place_piece() -> Result<(), BoardError> {
    let board = Board::default();
    let tavern = Piece::new_tavern(Team::White);
    board.can_place_piece(&tavern, (0, 0).into())?;

    board.can_place_piece(&tavern, (9, 9).into())?;

    assert!(matches!(
      board.can_place_piece(&tavern, (10, 0).into()),
      Err(BoardError::PieceOutOfBounds(_))
    ));

    assert!(matches!(
      board.can_place_piece(&tavern, (0, 10).into()),
      Err(BoardError::PieceOutOfBounds(_))
    ));

    let cathedral = Piece::new_cathedral();
    board.can_place_piece(&cathedral, (0, 0).into())?;

    board.can_place_piece(&cathedral, (6, 7).into())?;

    assert!(matches!(
      board.can_place_piece(&cathedral, (7, 7).into()),
      Err(BoardError::PieceOutOfBounds(_))
    ));

    assert!(matches!(
      board.can_place_piece(&cathedral, (8, 6).into()),
      Err(BoardError::PieceOutOfBounds(_))
    ));

    Ok(())
  }

  #[test]
  fn test_try_place_piece() -> Result<(), BoardError> {
    let mut board = Board::default();

    let inn = Piece::new_inn(Team::White);
    assert!(matches!(
      board
        .try_place_piece(inn, (0, 9).into())
        .expect_err("must be error"),
      BoardError::PieceOutOfBounds(_)
    ));

    let white_tavern = Piece::new_tavern(Team::White);
    board.try_place_piece(white_tavern, (1, 2).into())?;

    let black_tavern = Piece::new_tavern(Team::Black);
    assert!(matches!(
      board
        .try_place_piece(black_tavern, (1, 2).into())
        .expect_err("must be error"),
      BoardError::PieceOnOccupiedTile(_)
    ));

    let black_infirmary = Piece::new_infirmary(Team::Black);
    assert!(matches!(
      board
        .try_place_piece(black_infirmary, (1, 1).into())
        .expect_err("must be error"),
      BoardError::PieceOnOccupiedTile(_)
    ));

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

    board.try_place_piece(w_tavern1, (0, 0).into())?;
    board.try_place_piece(w_abbey, (0, 0).into())?;
    board.try_place_piece(w_stable1, (0, 3).into())?;
    w_stable2.rotate_clockwise();
    board.try_place_piece(w_stable2, (0, 4).into())?;
    w_academy.rotate_clockwise();
    board.try_place_piece(w_academy, (0, 5).into())?;
    board.try_place_piece(w_square, (0, 7).into())?;
    board.try_place_piece(w_tavern2, (2, 0).into())?;
    w_manor.rotate_clockwise();
    w_manor.rotate_clockwise();
    board.try_place_piece(w_manor, (1, 1).into())?;
    w_tower.rotate_counterclockwise();
    board.try_place_piece(w_tower, (1, 4).into())?;
    board.try_place_piece(w_inn1, (3, 0).into())?;
    board.try_place_piece(w_infirmary, (3, 1).into())?;
    w_castle.rotate_clockwise();
    board.try_place_piece(w_castle, (3, 3).into())?;
    board.try_place_piece(w_bridge, (5, 0).into())?;
    w_inn2.rotate_counterclockwise();
    board.try_place_piece(w_inn2, (5, 1).into())?;

    board.try_place_piece(b_bridge, (0, 9).into())?;
    board.try_place_piece(b_tavern1, (2, 8).into())?;
    b_manor.rotate_clockwise();
    b_manor.rotate_clockwise();
    board.try_place_piece(b_manor, (3, 6).into())?;
    b_castle.rotate_clockwise();
    board.try_place_piece(b_castle, (3, 8).into())?;
    b_inn1.rotate_counterclockwise();
    board.try_place_piece(b_inn1, (4, 5).into())?;
    board.try_place_piece(b_infirmary, (6, 2).into())?;
    b_tower.rotate_clockwise();
    board.try_place_piece(b_tower, (6, 4).into())?;
    b_abbey.rotate_clockwise();
    board.try_place_piece(b_abbey, (6, 8).into())?;
    b_academy.rotate_clockwise();
    b_academy.rotate_clockwise();
    board.try_place_piece(b_academy, (7, 0).into())?;
    board.try_place_piece(b_square, (8, 4).into())?;
    b_stable1.rotate_clockwise();
    board.try_place_piece(b_stable1, (9, 0).into())?;
    board.try_place_piece(b_tavern2, (9, 3).into())?;
    b_stable2.rotate_clockwise();
    board.try_place_piece(b_stable2, (9, 6).into())?;
    b_inn2.rotate_clockwise();
    b_inn2.rotate_clockwise();
    board.try_place_piece(b_inn2, (8, 8).into())?;

    board.try_place_piece(cathedral, (5, 6).into())?;

    assert!(board.tiles.iter().all(|t| matches!(t, Tile::Occupied(_))));

    let piece_pos = board.pieces.clone().into_iter().collect::<Vec<_>>();
    for (pos, _) in piece_pos.into_iter() {
      board.try_remove_piece(pos)?;
    }
    assert!(board
      .tiles
      .iter()
      .all(|t| matches!(t, Tile::Empty(Team::None))));

    Ok(())
  }

  #[test]
  fn test_find_tile_sets() {
    let mut board = Board::default();
    let piece = Piece::new_inn(Team::White);
    board.place_piece(piece.clone(), (8, 8).into());
    let piece = piece.placed_at((8, 8).into());

    let tile_sets = board.find_tile_sets(&piece);
    assert_eq!(tile_sets.len(), 2);
    assert_eq!(
      tile_sets.iter().map(HashSet::len).collect::<Vec<_>>(), //
      &[96, 1]
    );
  }
}
