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
  fn from(value: Color) -> RaylibColor {
    use Color::*;
    match value {
      LightGray => RaylibColor {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
      },
      Gray => RaylibColor {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
      },
      DarkGray => RaylibColor {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
      },
      Yellow => RaylibColor {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
      },
      Gold => RaylibColor {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
      },
      Orange => RaylibColor {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
      },
      Pink => RaylibColor {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
      },
      Red => RaylibColor {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
      },
      Maroon => RaylibColor {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
      },
      Green => RaylibColor {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
      },
      Lime => RaylibColor {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
      },
      DarkGreen => RaylibColor {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
      },
      SkyBlue => RaylibColor {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
      },
      Blue => RaylibColor {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
      },
      DarkBlue => RaylibColor {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
      },
      Purple => RaylibColor {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
      },
      Violet => RaylibColor {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
      },
      DarkPurple => RaylibColor {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
      },
      Beige => RaylibColor {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
      },
      Brown => RaylibColor {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
      },
      DarkBrown => RaylibColor {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
      },
      White => RaylibColor {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      },
      Black => RaylibColor {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      },
      Blank => RaylibColor {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      },
      Magenta => RaylibColor {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
      },
      RayWhite => RaylibColor {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
      },
      Custom(r, g, b, a) => RaylibColor { r, g, b, a },
    }
  }
}

impl Random for Color {
  fn random(rng: &mut impl Rng) -> Self {
    Self::Custom(rng.random(), rng.random(), rng.random(), 255)
  }
}
