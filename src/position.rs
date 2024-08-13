use std::ops::Add;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

impl Position {
  /// Checked integer coordinate addition.
  /// Computes `self + rhs`, returning `None` if overflow occurred.
  pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    Some(Self {
      x: match self.x.checked_add(rhs.x) {
        Some(it) => it,
        None => return None,
      },
      y: match self.y.checked_add(rhs.y) {
        Some(it) => it,
        None => return None,
      },
    })
  }

  // pub const fn checked_add(self, rhs: Self) -> Option<Self>
  /// Returns iterator of orthogonal and diagonal adjacent positions to given
  /// `position`. Positions with coordinates that are equal or bigger than
  /// those of `upper_bound` position or less than 0 are not returned.
  pub fn diagonal_adjacent_positions_iter(
    self,
    upper_bound: Self,
  ) -> impl Iterator<Item = Self> {
    self.adjacent_positions_iter(upper_bound, &[
      (0, 1),
      (1, 1),
      (1, 0),
      (1, -1),
      (0, -1),
      (-1, -1),
      (-1, 0),
      (-1, 1),
    ])
  }

  /// Returns iterator of orthogonal adjacent positions to given `position`.
  /// Positions with coordinates that are equal or bigger than those of
  /// `upper_bound` position or less than 0 are not returned.
  pub fn orthogonal_adjacent_positions_iter(
    self,
    upper_bound: Self,
  ) -> impl Iterator<Item = Self> {
    self.adjacent_positions_iter(upper_bound, &[
      (0, 1), //
      (1, 0),
      (0, -1),
      (-1, 0),
    ])
  }

  /// Returns iterator of adjacent positions to given `position`.
  /// Coordinates of adjacent positions are passed with `coords` argument.
  /// Positions with coordinates that are equal or bigger than those of
  /// `upper_bound` position or less than 0 are not returned.
  fn adjacent_positions_iter(
    self,
    upper_bound: Self,
    coords: &[(isize, isize)],
  ) -> impl Iterator<Item = Self> + '_ {
    coords.iter().filter_map(move |t| {
      let x = match self.x.overflowing_add_signed(t.0) {
        (_, true) => return None,
        (v, _) => v,
      };
      let y = match self.y.overflowing_add_signed(t.1) {
        (_, true) => return None,
        (v, _) => v,
      };
      if x >= upper_bound.x || y >= upper_bound.y {
        return None;
      }
      Some(Self { x, y })
    })
  }
}

impl From<(usize, usize)> for Position {
  fn from(value: (usize, usize)) -> Self {
    Self {
      x: value.0,
      y: value.1,
    }
  }
}

impl Add for Position {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;

  #[test]
  fn test_orthogonal_adjacent_positions() {
    let upper_bound = Position { x: 10, y: 10 };

    let p = Position { x: 1, y: 1 };
    let positions = p
      .orthogonal_adjacent_positions_iter(upper_bound)
      .collect::<HashSet<_>>();
    assert_eq!(
      positions,
      HashSet::from_iter([(1, 2), (2, 1), (1, 0), (0, 1)].map(Position::from))
    );

    let p = Position { x: 0, y: 0 };
    let positions = p
      .orthogonal_adjacent_positions_iter(upper_bound)
      .collect::<HashSet<_>>();
    assert_eq!(
      positions,
      HashSet::from_iter([(1, 0), (0, 1)].map(Position::from))
    );

    let p = Position { x: 9, y: 9 };
    let positions = p
      .orthogonal_adjacent_positions_iter(upper_bound)
      .collect::<HashSet<_>>();
    assert_eq!(
      positions,
      HashSet::from_iter([(9, 8), (8, 9)].map(Position::from))
    );
  }

  #[test]
  fn test_diagonal_adjacent_positions() {
    let upper_bound = Position { x: 10, y: 10 };

    let p = Position { x: 1, y: 1 };
    let positions = p
      .diagonal_adjacent_positions_iter(upper_bound)
      .collect::<HashSet<_>>();
    assert_eq!(
      positions,
      HashSet::from_iter(
        [
          (0, 0),
          (1, 0),
          (2, 0),
          (2, 1),
          (2, 2),
          (1, 2),
          (0, 2),
          (0, 1),
        ]
        .into_iter()
        .map(Position::from)
      )
    );

    let p = Position { x: 0, y: 0 };
    let positions = p
      .diagonal_adjacent_positions_iter(upper_bound)
      .collect::<HashSet<_>>();
    assert_eq!(
      positions,
      HashSet::from_iter([(1, 0), (0, 1), (1, 1)].map(Position::from))
    );

    let p = Position { x: 9, y: 9 };
    let positions = p
      .diagonal_adjacent_positions_iter(upper_bound)
      .collect::<HashSet<_>>();
    assert_eq!(
      positions,
      HashSet::from_iter([(9, 8), (8, 9), (8, 8)].map(Position::from))
    );
  }
}
