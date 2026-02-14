use {
  crate::{
    utils::Random,
    zappy::{Error, Result, constants::ITEM_KINDS},
  },
  rand::{Rng, RngExt},
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Ore {
  Linemate,
  Deraumere,
  Sibur,
  Mendiane,
  Phiras,
  Thystame,
}

#[derive(Clone)]
pub enum Item {
  Nourriture,
  Ore(Ore),
}

impl TryFrom<u8> for Ore {
  type Error = Error;
  fn try_from(value: u8) -> Result<Self> {
    use Ore::*;
    match value {
      0..=5 => Ok(match value {
        0 => Linemate,
        1 => Deraumere,
        2 => Sibur,
        3 => Mendiane,
        4 => Phiras,
        5 => Thystame,
        _ => Linemate,
      }),
      _ => Err(Error::InvalidOreNumber),
    }
  }
}

impl Random for Item {
  fn random(rng: &mut impl Rng) -> Self
  where
    Self: Sized,
  {
    use Item::*;

    match rng.random::<u8>() % ITEM_KINDS as u8 {
      0 => Nourriture,
      x => Ore(unsafe { self::Ore::try_from(x - 1).unwrap_unchecked() }),
    }
  }
}
