use {
  crate::raylib::bindings::{
    BeginDrawing, BeginMode3D, Camera3D as RaylibCamera3D, CameraMode_CAMERA_CUSTOM,
    CameraMode_CAMERA_FIRST_PERSON, CameraMode_CAMERA_FREE, CameraMode_CAMERA_ORBITAL,
    CameraMode_CAMERA_THIRD_PERSON, CameraMoveRight, CameraMoveUp,
    CameraProjection_CAMERA_ORTHOGRAPHIC, CameraProjection_CAMERA_PERSPECTIVE, ClearBackground,
    CloseWindow, Color as RaylibColor, DisableCursor, DrawCube, DrawCubeWires, DrawGrid, DrawModel,
    DrawText, EnableCursor, EndDrawing, EndMode3D, GetCurrentMonitor, GetMonitorRefreshRate,
    InitWindow, IsKeyDown, KeyboardKey_KEY_A, KeyboardKey_KEY_APOSTROPHE, KeyboardKey_KEY_B,
    KeyboardKey_KEY_BACK, KeyboardKey_KEY_BACKSLASH, KeyboardKey_KEY_BACKSPACE, KeyboardKey_KEY_C,
    KeyboardKey_KEY_CAPS_LOCK, KeyboardKey_KEY_COMMA, KeyboardKey_KEY_D, KeyboardKey_KEY_DELETE,
    KeyboardKey_KEY_DOWN, KeyboardKey_KEY_E, KeyboardKey_KEY_EIGHT, KeyboardKey_KEY_END,
    KeyboardKey_KEY_ENTER, KeyboardKey_KEY_EQUAL, KeyboardKey_KEY_ESCAPE, KeyboardKey_KEY_F,
    KeyboardKey_KEY_F1, KeyboardKey_KEY_F2, KeyboardKey_KEY_F3, KeyboardKey_KEY_F4,
    KeyboardKey_KEY_F5, KeyboardKey_KEY_F6, KeyboardKey_KEY_F7, KeyboardKey_KEY_F8,
    KeyboardKey_KEY_F9, KeyboardKey_KEY_F10, KeyboardKey_KEY_F11, KeyboardKey_KEY_F12,
    KeyboardKey_KEY_FIVE, KeyboardKey_KEY_FOUR, KeyboardKey_KEY_G, KeyboardKey_KEY_GRAVE,
    KeyboardKey_KEY_H, KeyboardKey_KEY_HOME, KeyboardKey_KEY_I, KeyboardKey_KEY_INSERT,
    KeyboardKey_KEY_J, KeyboardKey_KEY_K, KeyboardKey_KEY_KB_MENU, KeyboardKey_KEY_KP_0,
    KeyboardKey_KEY_KP_1, KeyboardKey_KEY_KP_2, KeyboardKey_KEY_KP_3, KeyboardKey_KEY_KP_4,
    KeyboardKey_KEY_KP_5, KeyboardKey_KEY_KP_6, KeyboardKey_KEY_KP_7, KeyboardKey_KEY_KP_8,
    KeyboardKey_KEY_KP_9, KeyboardKey_KEY_KP_ADD, KeyboardKey_KEY_KP_DECIMAL,
    KeyboardKey_KEY_KP_DIVIDE, KeyboardKey_KEY_KP_ENTER, KeyboardKey_KEY_KP_EQUAL,
    KeyboardKey_KEY_KP_MULTIPLY, KeyboardKey_KEY_KP_SUBTRACT, KeyboardKey_KEY_L,
    KeyboardKey_KEY_LEFT, KeyboardKey_KEY_LEFT_ALT, KeyboardKey_KEY_LEFT_BRACKET,
    KeyboardKey_KEY_LEFT_CONTROL, KeyboardKey_KEY_LEFT_SHIFT, KeyboardKey_KEY_LEFT_SUPER,
    KeyboardKey_KEY_M, KeyboardKey_KEY_MENU, KeyboardKey_KEY_MINUS, KeyboardKey_KEY_N,
    KeyboardKey_KEY_NINE, KeyboardKey_KEY_NULL, KeyboardKey_KEY_NUM_LOCK, KeyboardKey_KEY_O,
    KeyboardKey_KEY_ONE, KeyboardKey_KEY_P, KeyboardKey_KEY_PAGE_DOWN, KeyboardKey_KEY_PAGE_UP,
    KeyboardKey_KEY_PAUSE, KeyboardKey_KEY_PERIOD, KeyboardKey_KEY_PRINT_SCREEN, KeyboardKey_KEY_Q,
    KeyboardKey_KEY_R, KeyboardKey_KEY_RIGHT, KeyboardKey_KEY_RIGHT_ALT,
    KeyboardKey_KEY_RIGHT_BRACKET, KeyboardKey_KEY_RIGHT_CONTROL, KeyboardKey_KEY_RIGHT_SHIFT,
    KeyboardKey_KEY_RIGHT_SUPER, KeyboardKey_KEY_S, KeyboardKey_KEY_SCROLL_LOCK,
    KeyboardKey_KEY_SEMICOLON, KeyboardKey_KEY_SEVEN, KeyboardKey_KEY_SIX, KeyboardKey_KEY_SLASH,
    KeyboardKey_KEY_SPACE, KeyboardKey_KEY_T, KeyboardKey_KEY_TAB, KeyboardKey_KEY_THREE,
    KeyboardKey_KEY_TWO, KeyboardKey_KEY_U, KeyboardKey_KEY_UP, KeyboardKey_KEY_V,
    KeyboardKey_KEY_VOLUME_DOWN, KeyboardKey_KEY_VOLUME_UP, KeyboardKey_KEY_W, KeyboardKey_KEY_X,
    KeyboardKey_KEY_Y, KeyboardKey_KEY_Z, KeyboardKey_KEY_ZERO, Model as RaylibModel, SetTargetFPS,
    UpdateCamera, WindowShouldClose,
  },
  std::ffi::{CString, NulError, c_int},
  thiserror::Error,
};

pub use crate::raylib::bindings::{Vector2, Vector3, Vector4};

#[derive(Error, Debug)]
pub enum InitWindowError {
  #[error("nul character in title")]
  TitleContainsNul(#[from] NulError),
}

#[derive(Error, Debug)]
pub enum DrawTextError {
  #[error("nul character in text")]
  TextContainsNul(#[from] NulError),
}

#[derive(Error, Debug)]
pub enum Camera3DError {
  #[error("invalid camera projection value: {0}")]
  InvalidProjection(u32),
  #[error("invalid camera mode value: {0}")]
  InvalidMode(u32),
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum KeyboardKey {
  Null,
  Apostrophe,
  Comma,
  Minus,
  Period,
  Slash,
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Semicolon,
  Equal,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Left_bracket,
  Backslash,
  Right_bracket,
  Grave,
  Space,
  Escape,
  Enter,
  Tab,
  Backspace,
  Insert,
  Delete,
  Right,
  Left,
  Down,
  Up,
  Page_up,
  Page_down,
  Home,
  End,
  Caps_lock,
  Scroll_lock,
  Num_lock,
  Print_screen,
  Pause,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  Left_shift,
  Left_control,
  Left_alt,
  Left_super,
  Right_shift,
  Right_control,
  Right_alt,
  Right_super,
  Kb_menu,
  Kp_0,
  Kp_1,
  Kp_2,
  Kp_3,
  Kp_4,
  Kp_5,
  Kp_6,
  Kp_7,
  Kp_8,
  Kp_9,
  Kp_decimal,
  Kp_divide,
  Kp_multiply,
  Kp_subtract,
  Kp_add,
  Kp_enter,
  Kp_equal,
  Back,
  Menu,
  Volume_up,
  Volume_down,
  Other(u32),
}

#[derive(Clone, Debug)]
pub enum Camera3DProjection {
  Perspective,
  Orthographic,
}

#[derive(Clone)]
pub enum Camera3DMode {
  Free,
  Custom,
  Orbital,
  FirstPerson,
  ThirdPerson,
}

pub struct Window {
  title: CString,
}

pub struct Monitor {
  id: i32,
}

pub struct Pen {
  __: (),
}

pub struct Pen3D {
  __: (),
}

#[derive(Clone, Debug)]
pub struct Camera3D {
  pub position: Vector3,
  pub target: Vector3,
  pub up: Vector3,
  pub fovy: f32,
  pub projection: Camera3DProjection,
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

impl From<KeyboardKey> for u32 {
  fn from(value: KeyboardKey) -> Self {
    use KeyboardKey::*;
    match value {
      Null => KeyboardKey_KEY_NULL,
      Apostrophe => KeyboardKey_KEY_APOSTROPHE,
      Comma => KeyboardKey_KEY_COMMA,
      Minus => KeyboardKey_KEY_MINUS,
      Period => KeyboardKey_KEY_PERIOD,
      Slash => KeyboardKey_KEY_SLASH,
      Zero => KeyboardKey_KEY_ZERO,
      One => KeyboardKey_KEY_ONE,
      Two => KeyboardKey_KEY_TWO,
      Three => KeyboardKey_KEY_THREE,
      Four => KeyboardKey_KEY_FOUR,
      Five => KeyboardKey_KEY_FIVE,
      Six => KeyboardKey_KEY_SIX,
      Seven => KeyboardKey_KEY_SEVEN,
      Eight => KeyboardKey_KEY_EIGHT,
      Nine => KeyboardKey_KEY_NINE,
      Semicolon => KeyboardKey_KEY_SEMICOLON,
      Equal => KeyboardKey_KEY_EQUAL,
      A => KeyboardKey_KEY_A,
      B => KeyboardKey_KEY_B,
      C => KeyboardKey_KEY_C,
      D => KeyboardKey_KEY_D,
      E => KeyboardKey_KEY_E,
      F => KeyboardKey_KEY_F,
      G => KeyboardKey_KEY_G,
      H => KeyboardKey_KEY_H,
      I => KeyboardKey_KEY_I,
      J => KeyboardKey_KEY_J,
      K => KeyboardKey_KEY_K,
      L => KeyboardKey_KEY_L,
      M => KeyboardKey_KEY_M,
      N => KeyboardKey_KEY_N,
      O => KeyboardKey_KEY_O,
      P => KeyboardKey_KEY_P,
      Q => KeyboardKey_KEY_Q,
      R => KeyboardKey_KEY_R,
      S => KeyboardKey_KEY_S,
      T => KeyboardKey_KEY_T,
      U => KeyboardKey_KEY_U,
      V => KeyboardKey_KEY_V,
      W => KeyboardKey_KEY_W,
      X => KeyboardKey_KEY_X,
      Y => KeyboardKey_KEY_Y,
      Z => KeyboardKey_KEY_Z,
      Left_bracket => KeyboardKey_KEY_LEFT_BRACKET,
      Backslash => KeyboardKey_KEY_BACKSLASH,
      Right_bracket => KeyboardKey_KEY_RIGHT_BRACKET,
      Grave => KeyboardKey_KEY_GRAVE,
      Space => KeyboardKey_KEY_SPACE,
      Escape => KeyboardKey_KEY_ESCAPE,
      Enter => KeyboardKey_KEY_ENTER,
      Tab => KeyboardKey_KEY_TAB,
      Backspace => KeyboardKey_KEY_BACKSPACE,
      Insert => KeyboardKey_KEY_INSERT,
      Delete => KeyboardKey_KEY_DELETE,
      Right => KeyboardKey_KEY_RIGHT,
      Left => KeyboardKey_KEY_LEFT,
      Down => KeyboardKey_KEY_DOWN,
      Up => KeyboardKey_KEY_UP,
      Page_up => KeyboardKey_KEY_PAGE_UP,
      Page_down => KeyboardKey_KEY_PAGE_DOWN,
      Home => KeyboardKey_KEY_HOME,
      End => KeyboardKey_KEY_END,
      Caps_lock => KeyboardKey_KEY_CAPS_LOCK,
      Scroll_lock => KeyboardKey_KEY_SCROLL_LOCK,
      Num_lock => KeyboardKey_KEY_NUM_LOCK,
      Print_screen => KeyboardKey_KEY_PRINT_SCREEN,
      Pause => KeyboardKey_KEY_PAUSE,
      F1 => KeyboardKey_KEY_F1,
      F2 => KeyboardKey_KEY_F2,
      F3 => KeyboardKey_KEY_F3,
      F4 => KeyboardKey_KEY_F4,
      F5 => KeyboardKey_KEY_F5,
      F6 => KeyboardKey_KEY_F6,
      F7 => KeyboardKey_KEY_F7,
      F8 => KeyboardKey_KEY_F8,
      F9 => KeyboardKey_KEY_F9,
      F10 => KeyboardKey_KEY_F10,
      F11 => KeyboardKey_KEY_F11,
      F12 => KeyboardKey_KEY_F12,
      Left_shift => KeyboardKey_KEY_LEFT_SHIFT,
      Left_control => KeyboardKey_KEY_LEFT_CONTROL,
      Left_alt => KeyboardKey_KEY_LEFT_ALT,
      Left_super => KeyboardKey_KEY_LEFT_SUPER,
      Right_shift => KeyboardKey_KEY_RIGHT_SHIFT,
      Right_control => KeyboardKey_KEY_RIGHT_CONTROL,
      Right_alt => KeyboardKey_KEY_RIGHT_ALT,
      Right_super => KeyboardKey_KEY_RIGHT_SUPER,
      Kb_menu => KeyboardKey_KEY_KB_MENU,
      Kp_0 => KeyboardKey_KEY_KP_0,
      Kp_1 => KeyboardKey_KEY_KP_1,
      Kp_2 => KeyboardKey_KEY_KP_2,
      Kp_3 => KeyboardKey_KEY_KP_3,
      Kp_4 => KeyboardKey_KEY_KP_4,
      Kp_5 => KeyboardKey_KEY_KP_5,
      Kp_6 => KeyboardKey_KEY_KP_6,
      Kp_7 => KeyboardKey_KEY_KP_7,
      Kp_8 => KeyboardKey_KEY_KP_8,
      Kp_9 => KeyboardKey_KEY_KP_9,
      Kp_decimal => KeyboardKey_KEY_KP_DECIMAL,
      Kp_divide => KeyboardKey_KEY_KP_DIVIDE,
      Kp_multiply => KeyboardKey_KEY_KP_MULTIPLY,
      Kp_subtract => KeyboardKey_KEY_KP_SUBTRACT,
      Kp_add => KeyboardKey_KEY_KP_ADD,
      Kp_enter => KeyboardKey_KEY_KP_ENTER,
      Kp_equal => KeyboardKey_KEY_KP_EQUAL,
      Back => KeyboardKey_KEY_BACK,
      Menu => KeyboardKey_KEY_MENU,
      Volume_up => KeyboardKey_KEY_VOLUME_UP,
      Volume_down => KeyboardKey_KEY_VOLUME_DOWN,
      Other(value) => value,
    }
  }
}

impl From<Camera3DProjection> for u32 {
  fn from(value: Camera3DProjection) -> Self {
    use Camera3DProjection::*;
    match value {
      Perspective => CameraProjection_CAMERA_PERSPECTIVE,
      Orthographic => CameraProjection_CAMERA_ORTHOGRAPHIC,
    }
  }
}

impl TryFrom<u32> for Camera3DProjection {
  type Error = Camera3DError;
  fn try_from(value: u32) -> Result<Self, Self::Error> {
    use Camera3DError::*;
    use Camera3DProjection::*;
    match value {
      CameraProjection_CAMERA_PERSPECTIVE => Ok(Perspective),
      CameraProjection_CAMERA_ORTHOGRAPHIC => Ok(Orthographic),
      value => Err(InvalidProjection(value)),
    }
  }
}

impl From<Camera3DMode> for u32 {
  fn from(value: Camera3DMode) -> Self {
    use Camera3DMode::*;
    match value {
      Free => CameraMode_CAMERA_FREE,
      Custom => CameraMode_CAMERA_CUSTOM,
      Orbital => CameraMode_CAMERA_ORBITAL,
      FirstPerson => CameraMode_CAMERA_FIRST_PERSON,
      ThirdPerson => CameraMode_CAMERA_THIRD_PERSON,
    }
  }
}

impl TryFrom<u32> for Camera3DMode {
  type Error = Camera3DError;
  fn try_from(value: u32) -> Result<Self, Self::Error> {
    use Camera3DError::*;
    use Camera3DMode::*;
    match value {
      CameraMode_CAMERA_FREE => Ok(Free),
      CameraMode_CAMERA_CUSTOM => Ok(Custom),
      CameraMode_CAMERA_ORBITAL => Ok(Orbital),
      CameraMode_CAMERA_FIRST_PERSON => Ok(FirstPerson),
      CameraMode_CAMERA_THIRD_PERSON => Ok(ThirdPerson),
      value => Err(InvalidMode(value)),
    }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      CloseWindow();
    }
  }
}

impl Window {
  pub fn title(&self) -> &str {
    unsafe { self.title.to_str().unwrap_unchecked() }
  }

  pub fn should_close(&self) -> bool {
    unsafe { WindowShouldClose() }
  }

  pub fn begin_drawing<F: FnOnce(Pen)>(&self, callback: F) {
    unsafe { BeginDrawing() }
    callback(Pen { __: () });
  }

  pub fn enable_cursor(&self) {
    unsafe { EnableCursor() }
  }

  pub fn disable_cursor(&self) {
    unsafe { DisableCursor() }
  }

  pub fn set_target_fps(&self, fps: i32) {
    unsafe { SetTargetFPS(fps as c_int) }
  }
}

impl Drop for Pen {
  fn drop(&mut self) {
    unsafe {
      EndDrawing();
    }
  }
}

impl Pen {
  pub fn clear_background(&self, color: Color) {
    unsafe { ClearBackground(color.into()) }
  }

  pub fn draw_text(
    &self,
    text: &str,
    pos_x: i32,
    pos_y: i32,
    font_size: i32,
    color: Color,
  ) -> Result<(), DrawTextError> {
    let text = CString::new(text)?;

    unsafe { DrawText(text.as_ptr(), pos_x, pos_y, font_size, color.into()) }
    Ok(())
  }

  pub fn begin_mode_3d<F: FnOnce(Pen3D)>(&self, camera: &Camera3D, callback: F) {
    unsafe { BeginMode3D(camera.clone().into()) }
    callback(Pen3D { __: () });
  }
}

impl TryFrom<RaylibCamera3D> for Camera3D {
  type Error = Camera3DError;
  fn try_from(value: RaylibCamera3D) -> Result<Self, Self::Error> {
    Ok(Camera3D {
      position: value.position,
      target: value.target,
      up: value.up,
      fovy: value.fovy,
      projection: Camera3DProjection::try_from(value.projection as u32)?,
    })
  }
}

impl From<Camera3D> for RaylibCamera3D {
  fn from(value: Camera3D) -> Self {
    RaylibCamera3D {
      position: value.position,
      target: value.target,
      up: value.up,
      fovy: value.fovy,
      projection: u32::from(value.projection) as i32,
    }
  }
}

impl Camera3D {
  pub fn move_up(&mut self, distance: f32) {
    let mut cam = RaylibCamera3D::from(self.clone());
    unsafe { CameraMoveUp(&mut cam as *mut RaylibCamera3D, distance) }
    *self = unsafe { cam.try_into().unwrap_unchecked() }
  }

  pub fn move_right(&mut self, distance: f32, move_in_world_plane: bool) {
    let mut cam = RaylibCamera3D::from(self.clone());
    unsafe {
      CameraMoveRight(
        &mut cam as *mut RaylibCamera3D,
        distance,
        move_in_world_plane,
      )
    }
    *self = unsafe { cam.try_into().unwrap_unchecked() }
  }

  pub fn update(&mut self, mode: Camera3DMode) {
    let mut cam = RaylibCamera3D::from(self.clone());
    unsafe { UpdateCamera(&mut cam as *mut RaylibCamera3D, u32::from(mode) as c_int) }
    *self = unsafe { cam.try_into().unwrap_unchecked() }
  }
}

impl Monitor {
  pub fn get_refresh_rate(&self) -> i32 {
    (unsafe { GetMonitorRefreshRate(self.id as c_int) }) as i32
  }
}

impl Drop for Pen3D {
  fn drop(&mut self) {
    unsafe { EndMode3D() }
  }
}

impl Pen3D {
  pub fn draw_grid(&self, slices: i32, spacing: f32) {
    unsafe { DrawGrid(slices as c_int, spacing) }
  }

  pub fn draw_cube(&self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe { DrawCube(position, width, height, length, color.into()) }
  }

  pub fn draw_mesh(&self, model: RaylibModel, position: Vector3, scale: f32, tint: Color) {
    unsafe {
      DrawModel(model, position, scale, tint.into());
    }
  }

  pub fn draw_cube_wires(
    &self,
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
  ) {
    unsafe { DrawCubeWires(position, width, height, length, color.into()) }
  }
}

pub fn init_window(width: i32, height: i32, title: &str) -> Result<Window, InitWindowError> {
  let title = CString::new(title)?;

  unsafe { InitWindow(width as c_int, height as c_int, title.as_ptr()) }
  Ok(Window { title })
}

pub fn get_current_monitor() -> Monitor {
  Monitor {
    id: unsafe { GetCurrentMonitor() } as i32,
  }
}

pub fn is_key_down(key: KeyboardKey) -> bool {
  unsafe { IsKeyDown(u32::from(key) as c_int) }
}
