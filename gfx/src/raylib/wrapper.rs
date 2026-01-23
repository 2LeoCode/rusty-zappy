use {
  super::bindings::InitWindow,
  crate::raylib::bindings::{
    BeginDrawing, ClearBackground, CloseWindow, Color, EndDrawing, SetTargetFPS, WindowShouldClose,
  },
  std::ffi::{CString, NulError, c_int},
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum InitWindowError {
  #[error("nul character in title")]
  TitleContainsNul(#[from] NulError),
}

pub enum RaylibColor {
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
}

impl From<RaylibColor> for Color {
  fn from(value: RaylibColor) -> Color {
    use RaylibColor::*;
    match value {
      LightGray => Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
      },
      Gray => Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
      },
      DarkGray => Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
      },
      Yellow => Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
      },
      Gold => Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
      },
      Orange => Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
      },
      Pink => Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
      },
      Red => Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
      },
      Maroon => Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
      },
      Green => Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
      },
      Lime => Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
      },
      DarkGreen => Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
      },
      SkyBlue => Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
      },
      Blue => Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
      },
      DarkBlue => Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
      },
      Purple => Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
      },
      Violet => Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
      },
      DarkPurple => Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
      },
      Beige => Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
      },
      Brown => Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
      },
      DarkBrown => Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
      },
      White => Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      },
      Black => Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      },
      Blank => Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      },
      Magenta => Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
      },
      RayWhite => Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
      },
    }
  }
}

pub struct Window;
pub struct Drawer;

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      CloseWindow();
    }
  }
}

impl Drop for Drawer {
  fn drop(&mut self) {
    unsafe {
      EndDrawing();
    }
  }
}

impl Window {
  pub fn should_close(&self) -> bool {
    unsafe { WindowShouldClose() }
  }

  pub fn begin_drawing(&self) -> Drawer {
    unsafe {
      BeginDrawing();
    }
    Drawer {}
  }

  pub fn set_target_fps(&self, fps: i32) {
    unsafe {
      SetTargetFPS(fps as c_int);
    }
  }
}

impl Drawer {
  pub fn clear_background(&self, color: RaylibColor) {
    unsafe {
      ClearBackground(color.into());
    }
  }
}

pub fn init_window(width: i32, height: i32, title: &str) -> Result<Window, InitWindowError> {
  unsafe {
    InitWindow(width, height, CString::new(title)?.as_ptr());
  }
  Ok(Window {})
}
