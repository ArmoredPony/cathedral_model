use std::fmt::Display;

use ndarray::{array, Axis};

use super::*;

// TODO: make immutable, change position on placement, copy the entire thing
#[derive(Debug)]
pub struct Piece {
  pub position: (usize, usize),
  pub team: Team,
  pub layout: Array2<bool>,
}

impl Piece {
  pub fn new_tavern(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![[true]],
    }
  }

  pub fn new_stable(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [true], //
        [true]
      ],
    }
  }

  pub fn new_inn(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [true, true], //
        [true, false]
      ],
    }
  }

  pub fn new_bridge(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [true], //
        [true],
        [true]
      ],
    }
  }

  pub fn new_square(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [true, true], //
        [true, true]
      ],
    }
  }

  pub fn new_manor(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [true, true, true], //
        [false, true, false]
      ],
    }
  }

  pub fn new_abbey(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
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
    }
  }

  pub fn new_academy(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
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
    }
  }

  pub fn new_infirmary(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
      ],
    }
  }

  pub fn new_castle(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [true, true, true], //
        [true, false, true],
      ],
    }
  }

  pub fn new_tower(position: (usize, usize), team: Team) -> Self {
    Piece {
      position,
      team,
      layout: array![
        [false, true, true], //
        [true, true, false],
        [true, false, false],
      ],
    }
  }

  pub fn new_cathedral(position: (usize, usize)) -> Self {
    let team = Team::None;
    Piece {
      position,
      team,
      layout: array![
        [false, true, false], //
        [true, true, true],
        [false, true, false],
        [false, true, false],
      ],
    }
  }

  pub fn rotate_clockwise(&mut self) {
    self.layout.swap_axes(0, 1);
    self.layout.invert_axis(Axis(1));
  }

  pub fn rotate_counterclockwise(&mut self) {
    self.layout.swap_axes(0, 1);
    self.layout.invert_axis(Axis(0));
  }

  pub fn occupied_coords_iter(
    &self,
  ) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..self.layout.nrows())
      .flat_map(move |i| (0..self.layout.ncols()).map(move |j| (i, j)))
      .filter(|coords| self.layout[*coords])
  }
}

impl Display for Piece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.layout)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_clockwise_rotation() {
    let mut tavern = Piece::new_tavern((0, 0), Team::White);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_clockwise();
    assert_eq!(tavern.layout, array![[true]]);

    let mut stable = Piece::new_stable((0, 0), Team::White);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true], [true]]);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_clockwise();
    assert_eq!(stable.layout, array![[true], [true]]);

    let mut inn = Piece::new_inn((0, 0), Team::White);
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

    let mut manor = Piece::new_manor((0, 0), Team::White);
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

    let mut cathedral = Piece::new_cathedral((0, 0));
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
    let mut tavern = Piece::new_tavern((0, 0), Team::White);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);
    tavern.rotate_counterclockwise();
    assert_eq!(tavern.layout, array![[true]]);

    let mut stable = Piece::new_stable((0, 0), Team::White);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true], [true]]);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true, true]]);
    stable.rotate_counterclockwise();
    assert_eq!(stable.layout, array![[true], [true]]);

    let mut inn = Piece::new_inn((0, 0), Team::White);
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

    let mut manor = Piece::new_manor((0, 0), Team::White);
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

    let mut cathedral = Piece::new_cathedral((0, 0));
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
