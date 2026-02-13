use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("invalid ore number")]
  InvalidOreNumber,

  #[error("team '{0}' already exists")]
  TeamExists(String),

  #[error("team '{0}' doesn't exist")]
  TeamDoesntExist(String),

  #[error("team '{0}' is full")]
  TeamIsFull(String),

  #[error("player id {1} out of bounds for team '{0}'")]
  PlayerOutOfBounds(String, usize),

  #[error("no player with id {1} found in team '{0}'")]
  PlayerNotFound(String, usize),
}

pub type Result<T> = std::result::Result<T, Error>;
