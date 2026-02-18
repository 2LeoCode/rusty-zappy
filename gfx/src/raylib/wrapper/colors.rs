use common::utils::Random;
use rand::{Rng, RngExt};

use crate::raylib::bindings::Color as RaylibColor;

#[derive(Clone, Debug)]
pub enum Color {
  LightGray,
  Gray,
  DarkGray,
  Yellow,
  Gold,
  Orange,
  Pink,
  Red,
  Maroon,
  Green,
  Lime,
  DarkGreen,
  SkyBlue,
  Blue,
  DarkBlue,
  Purple,
  Violet,
  DarkPurple,
  Beige,
  Brown,
  DarkBrown,
  White,
  Black,
  Blank,
  Magenta,
  RayWhite,
  Custom(u8, u8, u8, u8),
}

impl From<Color> for RaylibColor {
  fn from(value: Color) -> Self {
    use Color::*;
    match value {
      LightGray => Self {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
      },
      Gray => Self {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
      },
      DarkGray => Self {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
      },
      Yellow => Self {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
      },
      Gold => Self {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
      },
      Orange => Self {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
      },
      Pink => Self {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
      },
      Red => Self {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
      },
      Maroon => Self {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
      },
      Green => Self {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
      },
      Lime => Self {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
      },
      DarkGreen => Self {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
      },
      SkyBlue => Self {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
      },
      Blue => Self {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
      },
      DarkBlue => Self {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
      },
      Purple => Self {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
      },
      Violet => Self {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
      },
      DarkPurple => Self {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
      },
      Beige => Self {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
      },
      Brown => Self {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
      },
      DarkBrown => Self {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
      },
      White => Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      },
      Black => Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      },
      Blank => Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      },
      Magenta => Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
      },
      RayWhite => Self {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
      },
      Custom(r, g, b, a) => Self { r, g, b, a },
    }
  }
}

impl Random for Color {
  fn random(rng: &mut impl Rng) -> Self {
    Self::Custom(rng.random(), rng.random(), rng.random(), 255)
  }
}
