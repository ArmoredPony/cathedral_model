use std::{fmt::Display, marker::PhantomData};

use ndarray::{array, Axis};

use super::*;

pub trait PieceState {}

pub enum Placed {}
impl PieceState for Placed {}

pub enum Released {}
impl PieceState for Released {}

// TODO: make immutable, change position on placement, copy the entire thing
#[derive(Debug)]
pub struct Piece<S: PieceState> {
  position: (usize, usize),
  pub team: Team,
  pub layout: Array2<bool>,
  _state: PhantomData<S>,
}

impl Piece<Released> {
  pub fn new_tavern(team: Team) -> Self {
    Piece {
      team,
      layout: array![[true]],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_stable(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true], //
        [true]
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_inn(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true], //
        [true, false]
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_bridge(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true], //
        [true],
        [true]
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_square(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true], //
        [true, true]
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_manor(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true, true], //
        [false, true, false]
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

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
      position: Default::default(),
      _state: PhantomData,
    }
  }

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
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_infirmary(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_castle(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [true, true, true], //
        [true, false, true],
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

  pub fn new_tower(team: Team) -> Self {
    Piece {
      team,
      layout: array![
        [false, true, true], //
        [true, true, false],
        [true, false, false],
      ],
      position: Default::default(),
      _state: PhantomData,
    }
  }

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
      position: Default::default(),
      _state: PhantomData,
    }
  }

  /// Rotates piece 90 degrees clockwise.
  pub fn rotate_clockwise(&mut self) {
    self.layout.swap_axes(0, 1);
    self.layout.invert_axis(Axis(1));
  }

  /// Rotates piece 90 degrees counterclockwise.
  pub fn rotate_counterclockwise(&mut self) {
    self.layout.swap_axes(0, 1);
    self.layout.invert_axis(Axis(0));
  }

  /// Returns iterator of tiles' local coordinates that this piece occupies.
  pub fn occupied_coords_iter(
    &self,
  ) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..self.layout.nrows())
      .flat_map(move |i| (0..self.layout.ncols()).map(move |j| (i, j)))
      .filter(|coords| self.layout[*coords])
  }

  pub fn placed_at(self, position: (usize, usize)) -> Piece<Placed> {
    Piece {
      position,
      team: self.team,
      layout: self.layout,
      _state: PhantomData,
    }
  }
}

impl Piece<Placed> {
  pub fn position(&self) -> (usize, usize) {
    self.position
  }
}

impl<S: PieceState> Display for Piece<S> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    assert_eq!(
      inn.layout,
      array![
        [true, true], //
        [false, true]
      ]
    );
    inn.rotate_clockwise();
    assert_eq!(
      inn.layout,
      array![
        [false, true], //
        [true, true]
      ]
    );
    inn.rotate_clockwise();
    assert_eq!(
      inn.layout,
      array![
        [true, false], //
        [true, true]
      ]
    );
    inn.rotate_clockwise();
    assert_eq!(
      inn.layout,
      array![
        [true, true], //
        [true, false]
      ]
    );

    let mut manor = Piece::new_manor(Team::White);
    manor.rotate_clockwise();
    assert_eq!(
      manor.layout,
      array![
        [false, true], //
        [true, true],
        [false, true]
      ]
    );
    manor.rotate_clockwise();
    assert_eq!(
      manor.layout,
      array![
        [false, true, false], //
        [true, true, true]
      ]
    );
    manor.rotate_clockwise();
    assert_eq!(
      manor.layout,
      array![
        [true, false], //
        [true, true],
        [true, false]
      ]
    );
    manor.rotate_clockwise();
    assert_eq!(
      manor.layout,
      array![
        [true, true, true], //
        [false, true, false]
      ]
    );

    let mut cathedral = Piece::new_cathedral();
    cathedral.rotate_clockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, false, true, false], //
        [true, true, true, true],
        [false, false, true, false]
      ]
    );
    cathedral.rotate_clockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, true, false], //
        [false, true, false],
        [true, true, true],
        [false, true, false],
      ]
    );
    cathedral.rotate_clockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, true, false, false], //
        [true, true, true, true],
        [false, true, false, false]
      ]
    );
    cathedral.rotate_clockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
        [false, true, false],
      ]
    );
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
    assert_eq!(
      inn.layout,
      array![
        [true, false], //
        [true, true]
      ]
    );
    inn.rotate_counterclockwise();
    assert_eq!(
      inn.layout,
      array![
        [false, true], //
        [true, true]
      ]
    );
    inn.rotate_counterclockwise();
    assert_eq!(
      inn.layout,
      array![
        [true, true], //
        [false, true]
      ]
    );
    inn.rotate_counterclockwise();
    assert_eq!(
      inn.layout,
      array![
        [true, true], //
        [true, false]
      ]
    );

    let mut manor = Piece::new_manor(Team::White);
    manor.rotate_counterclockwise();
    assert_eq!(
      manor.layout,
      array![
        [true, false], //
        [true, true],
        [true, false]
      ]
    );
    manor.rotate_counterclockwise();
    assert_eq!(
      manor.layout,
      array![
        [false, true, false], //
        [true, true, true]
      ]
    );
    manor.rotate_counterclockwise();
    assert_eq!(
      manor.layout,
      array![
        [false, true], //
        [true, true],
        [false, true]
      ]
    );
    manor.rotate_counterclockwise();
    assert_eq!(
      manor.layout,
      array![
        [true, true, true], //
        [false, true, false]
      ]
    );

    let mut cathedral = Piece::new_cathedral();
    cathedral.rotate_counterclockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, true, false, false], //
        [true, true, true, true],
        [false, true, false, false]
      ]
    );
    cathedral.rotate_counterclockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, true, false], //
        [false, true, false],
        [true, true, true],
        [false, true, false],
      ]
    );
    cathedral.rotate_counterclockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, false, true, false], //
        [true, true, true, true],
        [false, false, true, false]
      ]
    );
    cathedral.rotate_counterclockwise();
    assert_eq!(
      cathedral.layout,
      array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
        [false, true, false],
      ]
    );
  }
}
