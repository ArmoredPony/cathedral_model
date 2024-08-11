use std::{fmt::Display, marker::PhantomData};

use ndarray::{array, Axis};

use super::*;

pub trait PieceState {}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Placed {}
impl PieceState for Placed {}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Released {}
impl PieceState for Released {}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Rotation {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

impl Rotation {
  pub fn rotated_clockwise(self) -> Self {
    match self {
      Rotation::UP => Rotation::RIGHT,
      Rotation::RIGHT => Rotation::DOWN,
      Rotation::DOWN => Rotation::LEFT,
      Rotation::LEFT => Rotation::UP,
    }
  }

  pub fn rotated_counterclockwise(self) -> Self {
    match self {
      Rotation::UP => Rotation::LEFT,
      Rotation::LEFT => Rotation::DOWN,
      Rotation::DOWN => Rotation::RIGHT,
      Rotation::RIGHT => Rotation::UP,
    }
  }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Piece<S: PieceState> {
  team: Team,
  layout: Array2<bool>,
  position: Position,
  rotation: Rotation,
  _state: PhantomData<S>,
}

impl<S: PieceState> Piece<S> {
  pub fn team(&self) -> Team {
    self.team
  }

  /// Returns iterator of tiles' local coordinates that this piece occupies.
  pub fn occupied_coords_iter(&self) -> impl Iterator<Item = Position> + '_ {
    (0..self.layout.nrows())
      .flat_map(move |i| (0..self.layout.ncols()).map(move |j| (i, j)))
      .filter(|coords| self.layout[*coords])
      .map(Position::from)
  }
}

impl Piece<Released> {
  /// Returns a piece with this layout:
  /// <pre>
  /// []
  /// </pre>
  pub fn new_tavern(team: Team) -> Self {
    Piece {
      team,
      layout: array![[true]],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// []
  /// []
  /// </pre>
  pub fn new_stable(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true], //
        [true]
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// [][]
  /// []
  /// </pre>
  pub fn new_inn(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true], //
        [true, false]
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// []
  /// []
  /// []
  /// </pre>
  pub fn new_bridge(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true], //
        [true],
        [true]
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// [][]
  /// [][]
  /// </pre>
  pub fn new_square(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true], //
        [true, true]
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// [][][]
  ///   []
  /// </pre>
  pub fn new_manor(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true, true], //
        [false, true, false]
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// white:  black:
  ///   [][]  [][]
  /// [][]      [][]
  /// </pre>
  pub fn new_abbey(team: Team) -> Self {
    Piece {
      team,
      layout: match team {
        Team::White => array![
          [false, true, true], //
          [true, true, false]
        ],
        Team::Black => array![
          [true, true, false], //
          [false, true, true],
        ],
        _ => unreachable!("a piece can be either black or white"),
      },
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// white:  black:
  ///     []  []
  /// [][][]  [][][]
  ///   []      []
  /// </pre>
  pub fn new_academy(team: Team) -> Self {
    Piece {
      team,
      layout: match team {
        Team::White => array![
          [false, false, true], //
          [true, true, true],
          [false, true, false],
        ],
        Team::Black => array![
          [true, false, false], //
          [true, true, true],
          [false, true, false],
        ],
        _ => unreachable!("a piece can be either black or white"),
      },
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  ///   []
  /// [][][]
  ///   []
  /// </pre>
  pub fn new_infirmary(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  /// [][][]
  /// []  []
  /// </pre>
  pub fn new_castle(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true, true], //
        [true, false, true],
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  ///   [][]
  /// [][]
  /// []
  /// </pre>
  pub fn new_tower(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [false, true, true], //
        [true, true, false],
        [true, false, false],
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Returns a piece with this layout:
  /// <pre>
  ///   []
  /// [][][]
  ///   []
  ///   []
  /// </pre>
  pub fn new_cathedral() -> Self {
    let team = Team::None;
    Piece {
      team,
      layout: array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
        [false, true, false],
      ],
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }

  /// Rotates piece 90 degrees clockwise.
  pub fn rotate_clockwise(&mut self) {
    self.layout.swap_axes(0, 1);
    self.layout.invert_axis(Axis(1));
    self.rotation = self.rotation.clone().rotated_clockwise();
  }

  /// Rotates piece 90 degrees counterclockwise.
  pub fn rotate_counterclockwise(&mut self) {
    self.layout.swap_axes(0, 1);
    self.layout.invert_axis(Axis(0));
    self.rotation = self.rotation.clone().rotated_counterclockwise();
  }

  /// Emulates placing a piece down at given position.
  /// Changes its position and state to `Placed`.
  pub fn placed_at(self, position: Position) -> Piece<Placed> {
    Piece {
      team: self.team,
      layout: self.layout,
      position,
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }
}

impl Piece<Placed> {
  pub fn position(&self) -> Position {
    self.position
  }

  /// Emulates picking a piece up. Changes its state to `Released`.
  pub fn released(self) -> Piece<Released> {
    Piece {
      team: self.team,
      layout: self.layout,
      position: Position::default(),
      rotation: Rotation::UP,
      _state: PhantomData,
    }
  }
}

impl<S: PieceState> Display for Piece<S> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.layout)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_clockwise_rotation() {
    let mut tavern = Piece::new_tavern(Team::White);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);

    let mut stable = Piece::new_stable(Team::White);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true], [true]]);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true], [true]]);

    let mut inn = Piece::new_inn(Team::White);
    inn.rotate_clockwise();
    assert_eq!(inn.layout, array![
      [true, true], //
      [false, true]
    ]);
    inn.rotate_clockwise();
    assert_eq!(inn.layout, array![
      [false, true], //
      [true, true]
    ]);
    inn.rotate_clockwise();
    assert_eq!(inn.layout, array![
      [true, false], //
      [true, true]
    ]);
    inn.rotate_clockwise();
    assert_eq!(inn.layout, array![
      [true, true], //
      [true, false]
    ]);

    let mut manor = Piece::new_manor(Team::White);
    manor.rotate_clockwise();
    assert_eq!(manor.layout, array![
      [false, true], //
      [true, true],
      [false, true]
    ]);
    manor.rotate_clockwise();
    assert_eq!(manor.layout, array![
      [false, true, false], //
      [true, true, true]
    ]);
    manor.rotate_clockwise();
    assert_eq!(manor.layout, array![
      [true, false], //
      [true, true],
      [true, false]
    ]);
    manor.rotate_clockwise();
    assert_eq!(manor.layout, array![
      [true, true, true], //
      [false, true, false]
    ]);

    let mut cathedral = Piece::new_cathedral();
    cathedral.rotate_clockwise();
    assert_eq!(cathedral.layout, array![
      [false, false, true, false], //
      [true, true, true, true],
      [false, false, true, false]
    ]);
    cathedral.rotate_clockwise();
    assert_eq!(cathedral.layout, array![
      [false, true, false], //
      [false, true, false],
      [true, true, true],
      [false, true, false],
    ]);
    cathedral.rotate_clockwise();
    assert_eq!(cathedral.layout, array![
      [false, true, false, false], //
      [true, true, true, true],
      [false, true, false, false]
    ]);
    cathedral.rotate_clockwise();
    assert_eq!(cathedral.layout, array![
      [false, true, false], //
      [true, true, true],
      [false, true, false],
      [false, true, false],
    ]);
  }

  #[test]
  fn test_counterclockwise_rotation() {
    let mut tavern = Piece::new_tavern(Team::White);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);

    let mut stable = Piece::new_stable(Team::White);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true], [true]]);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true], [true]]);

    let mut inn = Piece::new_inn(Team::White);
    inn.rotate_counterclockwise();
    assert_eq!(inn.layout, array![
      [true, false], //
      [true, true]
    ]);
    inn.rotate_counterclockwise();
    assert_eq!(inn.layout, array![
      [false, true], //
      [true, true]
    ]);
    inn.rotate_counterclockwise();
    assert_eq!(inn.layout, array![
      [true, true], //
      [false, true]
    ]);
    inn.rotate_counterclockwise();
    assert_eq!(inn.layout, array![
      [true, true], //
      [true, false]
    ]);

    let mut manor = Piece::new_manor(Team::White);
    manor.rotate_counterclockwise();
    assert_eq!(manor.layout, array![
      [true, false], //
      [true, true],
      [true, false]
    ]);
    manor.rotate_counterclockwise();
    assert_eq!(manor.layout, array![
      [false, true, false], //
      [true, true, true]
    ]);
    manor.rotate_counterclockwise();
    assert_eq!(manor.layout, array![
      [false, true], //
      [true, true],
      [false, true]
    ]);
    manor.rotate_counterclockwise();
    assert_eq!(manor.layout, array![
      [true, true, true], //
      [false, true, false]
    ]);

    let mut cathedral = Piece::new_cathedral();
    cathedral.rotate_counterclockwise();
    assert_eq!(cathedral.layout, array![
      [false, true, false, false], //
      [true, true, true, true],
      [false, true, false, false]
    ]);
    cathedral.rotate_counterclockwise();
    assert_eq!(cathedral.layout, array![
      [false, true, false], //
      [false, true, false],
      [true, true, true],
      [false, true, false],
    ]);
    cathedral.rotate_counterclockwise();
    assert_eq!(cathedral.layout, array![
      [false, false, true, false], //
      [true, true, true, true],
      [false, false, true, false]
    ]);
    cathedral.rotate_counterclockwise();
    assert_eq!(cathedral.layout, array![
      [false, true, false], //
      [true, true, true],
      [false, true, false],
      [false, true, false],
    ]);
  }
}
