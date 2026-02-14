use {
  crate::raylib::{
    bindings::{
      BeginDrawing, Camera3D as RaylibCamera3D, CloseWindow, ConfigFlags as RaylibConfigFlags,
      ConfigFlags_FLAG_BORDERLESS_WINDOWED_MODE, ConfigFlags_FLAG_FULLSCREEN_MODE,
      ConfigFlags_FLAG_INTERLACED_HINT, ConfigFlags_FLAG_MSAA_4X_HINT, ConfigFlags_FLAG_VSYNC_HINT,
      ConfigFlags_FLAG_WINDOW_ALWAYS_RUN, ConfigFlags_FLAG_WINDOW_HIDDEN,
      ConfigFlags_FLAG_WINDOW_HIGHDPI, ConfigFlags_FLAG_WINDOW_MAXIMIZED,
      ConfigFlags_FLAG_WINDOW_MINIMIZED, ConfigFlags_FLAG_WINDOW_MOUSE_PASSTHROUGH,
      ConfigFlags_FLAG_WINDOW_RESIZABLE, ConfigFlags_FLAG_WINDOW_TOPMOST,
      ConfigFlags_FLAG_WINDOW_TRANSPARENT, ConfigFlags_FLAG_WINDOW_UNDECORATED,
      ConfigFlags_FLAG_WINDOW_UNFOCUSED, DisableCursor, EnableCursor, GetCurrentMonitor,
      InitWindow, SetConfigFlags, SetTargetFPS, SetWindowPosition, SetWindowSize, Vector3,
      WindowShouldClose, rlGetCullDistanceFar, rlGetCullDistanceNear,
    },
    wrapper::{
      cameras::{Camera3D, Camera3DMode, Camera3DProjection, ORTHO_CULL_FACTOR},
      monitor::Monitor,
      pens::Pen,
    },
  },
  std::{
    collections::HashSet,
    ffi::{CString, NulError, c_int},
  },
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum Error {
  #[error("nul character in window title")]
  TitleContainsNul(#[from] NulError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Eq, Hash, PartialEq)]
pub enum ConfigFlag {
  VsyncHint,
  FullscreenMode,
  WindowResizable,
  WindowUndecorated,
  WindowHidden,
  WindowMinimized,
  WindowMaximized,
  WindowUnfocused,
  WindowTopmost,
  WindowAlwaysRun,
  WindowTransparent,
  WindowHighdpi,
  WindowMousePassthrough,
  BorderlessWindowedMode,
  Msaa4xHint,
  InterlacedHint,
}

impl From<ConfigFlag> for RaylibConfigFlags {
  fn from(value: ConfigFlag) -> Self {
    use ConfigFlag::*;
    match value {
      VsyncHint => ConfigFlags_FLAG_VSYNC_HINT,
      FullscreenMode => ConfigFlags_FLAG_FULLSCREEN_MODE,
      WindowResizable => ConfigFlags_FLAG_WINDOW_RESIZABLE,
      WindowUndecorated => ConfigFlags_FLAG_WINDOW_UNDECORATED,
      WindowHidden => ConfigFlags_FLAG_WINDOW_HIDDEN,
      WindowMinimized => ConfigFlags_FLAG_WINDOW_MINIMIZED,
      WindowMaximized => ConfigFlags_FLAG_WINDOW_MAXIMIZED,
      WindowUnfocused => ConfigFlags_FLAG_WINDOW_UNFOCUSED,
      WindowTopmost => ConfigFlags_FLAG_WINDOW_TOPMOST,
      WindowAlwaysRun => ConfigFlags_FLAG_WINDOW_ALWAYS_RUN,
      WindowTransparent => ConfigFlags_FLAG_WINDOW_TRANSPARENT,
      WindowHighdpi => ConfigFlags_FLAG_WINDOW_HIGHDPI,
      WindowMousePassthrough => ConfigFlags_FLAG_WINDOW_MOUSE_PASSTHROUGH,
      BorderlessWindowedMode => ConfigFlags_FLAG_BORDERLESS_WINDOWED_MODE,
      Msaa4xHint => ConfigFlags_FLAG_MSAA_4X_HINT,
      InterlacedHint => ConfigFlags_FLAG_INTERLACED_HINT,
    }
  }
}

pub struct Builder {
  title: CString,
  flags: HashSet<ConfigFlag>,
}

#[derive(Debug)]
pub struct Window {
  pub(crate) width: u32,
  pub(crate) height: u32,
  title: CString,
}

impl Builder {
  pub fn try_new(title: impl Into<Vec<u8>>) -> Result<Self> {
    Ok(Self {
      title: CString::new(title)?,
      flags: Default::default(),
    })
  }

  pub fn add_config_flag(&mut self, flag: ConfigFlag) -> &mut Self {
    self.flags.insert(flag);
    self
  }

  pub fn build(self, width: u32, height: u32) -> Window {
    let mut flags: RaylibConfigFlags = 0;
    for flag in self.flags {
      flags |= RaylibConfigFlags::from(flag);
    }
    unsafe { SetConfigFlags(flags) }
    let instance = Window {
      title: self.title,
      width,
      height,
    };

    unsafe { InitWindow(width as c_int, height as c_int, instance.title.as_ptr()) }
    instance
  }
}

impl Window {
  pub fn set_size(&mut self, width: u32, height: u32) {
    unsafe { SetWindowSize(width as c_int, height as c_int) }
    self.width = width;
    self.height = height;
  }

  pub fn set_position(&mut self, x: u32, y: u32) {
    unsafe { SetWindowPosition(x as c_int, y as c_int) }
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

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

  pub fn new_camera_3d<'a>(
    &'a self,
    position: Vector3,
    target: Vector3,
    up: Vector3,
    fovy: f32,
    projection: Camera3DProjection,
  ) -> Camera3D<'a> {
    use Camera3DProjection::*;
    Camera3D {
      window: self,
      projection: projection.clone(),
      near: match projection {
        Perspective => unsafe { rlGetCullDistanceNear() },
        Orthographic => -(fovy as f64 * ORTHO_CULL_FACTOR),
      },
      far: match projection {
        Perspective => unsafe { rlGetCullDistanceFar() },
        Orthographic => fovy as f64 * ORTHO_CULL_FACTOR,
      },
      raylib_camera_3d: RaylibCamera3D {
        position,
        target,
        up,
        fovy,
        projection: u32::from(projection) as i32,
      },
    }
  }

  pub fn get_current_monitor(&self) -> Monitor {
    let id = unsafe { GetCurrentMonitor() } as i32;
    Monitor { id }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe {
      CloseWindow();
    }
  }
}
