use thiserror::Error;

use crate::position::Position;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BoardError {
  #[error("piece was placed out of bounds at {0:?}")]
  PieceOutOfBounds(Position),
  #[error("piece was placed on occupied tile")]
  PieceOnOccupiedTile(Position),
  #[error("piece was placed on other team's tile")]
  PieceOnEnemyTile(Position),
  #[error("place doesn't belong to this board")]
  PieceNotOnBoard,
}
