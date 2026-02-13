use {
  crate::raylib::{
    bindings::{
      BeginDrawing, Camera3D as RaylibCamera3D, CloseWindow, DisableCursor, EnableCursor,
      GetCurrentMonitor, InitWindow, SetTargetFPS, SetWindowPosition, SetWindowSize, Vector3,
      WindowShouldClose, rlGetCullDistanceFar, rlGetCullDistanceNear,
    },
    wrapper::{
      cameras::{Camera3D, Camera3DMode, Camera3DProjection},
      monitor::Monitor,
      pens::Pen,
    },
  },
  std::ffi::{CString, NulError, c_int},
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum Error {
  #[error("nul character in window title")]
  TitleContainsNul(#[from] NulError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Window {
  pub(crate) width: u32,
  pub(crate) height: u32,
  title: CString,
}

impl Window {
  pub fn try_init(width: u32, height: u32, title: impl Into<Vec<u8>>) -> Result<Self> {
    let title = CString::new(title)?;

    unsafe { InitWindow(width as c_int, height as c_int, title.as_ptr()) }
    Ok(Window {
      width,
      height,
      title,
    })
  }

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
        Orthographic => -(self.height as f32 * (fovy / 10.)) as f64 / 4.,
      },
      far: match projection {
        Perspective => unsafe { rlGetCullDistanceFar() },
        Orthographic => (self.height as f32 * (fovy / 10.)) as f64 / 4.,
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
