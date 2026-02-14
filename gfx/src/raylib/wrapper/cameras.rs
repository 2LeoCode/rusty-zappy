use {
  crate::raylib::{
    bindings::{
      Camera3D as RaylibCamera3D, CameraMode_CAMERA_CUSTOM, CameraMode_CAMERA_FIRST_PERSON,
      CameraMode_CAMERA_FREE, CameraMode_CAMERA_ORBITAL, CameraMode_CAMERA_THIRD_PERSON,
      CameraMoveForward, CameraMoveRight, CameraProjection_CAMERA_ORTHOGRAPHIC,
      CameraProjection_CAMERA_PERSPECTIVE, UpdateCamera,
    },
    wrapper::window::Window,
  },
  std::ffi::c_int,
  thiserror::Error,
};

pub(crate) const ORTHO_CULL_FACTOR: f64 = 0.7071067812;

#[derive(Error, Debug)]
pub enum Error {
  #[error("invalid camera projection value: {0}")]
  InvalidProjection(u32),
  #[error("invalid camera mode value: {0}")]
  InvalidMode(u32),
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
pub struct Camera3D<'a> {
  pub(crate) window: &'a Window,
  pub(crate) projection: Camera3DProjection,
  pub(crate) near: f64,
  pub(crate) far: f64,
  pub(crate) raylib_camera_3d: RaylibCamera3D,
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
  type Error = Error;
  fn try_from(value: u32) -> Result<Self, Self::Error> {
    use Camera3DProjection::*;
    use Error::*;
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
  type Error = Error;
  fn try_from(value: u32) -> Result<Self, Self::Error> {
    use Camera3DMode::*;
    use Error::*;
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

impl Camera3D<'_> {
  pub fn fovy(&self) -> f32 {
    self.raylib_camera_3d.fovy
  }

  pub fn set_fovy(&mut self, value: f32) {
    self.raylib_camera_3d.fovy = value;
    if self.projection == Camera3DProjection::Orthographic {
      self.far = ORTHO_CULL_FACTOR * value as f64;
      self.near = -self.far;
    }
  }

  pub fn move_up(&mut self, distance: f32) {
    let sign = -(distance / distance.abs());
    let offset = ((distance * distance) / 2.).sqrt() * sign;
    self.raylib_camera_3d.position.x += offset;
    self.raylib_camera_3d.position.z += offset;
    self.raylib_camera_3d.target.x += offset;
    self.raylib_camera_3d.target.z += offset;
  }

  pub fn move_right(&mut self, distance: f32, move_in_world_plane: bool) {
    let sign = -(distance / distance.abs());
    let ratio = self.window.height() as f32 / self.window.width() as f32;
    let offset = ((distance * distance) / 2.).sqrt() * sign * ratio;
    self.raylib_camera_3d.position.x -= offset;
    self.raylib_camera_3d.position.z += offset;
    self.raylib_camera_3d.target.x -= offset;
    self.raylib_camera_3d.target.z += offset;
  }

  pub fn update(&mut self, mode: Camera3DMode) {
    unsafe {
      UpdateCamera(
        &mut self.raylib_camera_3d as *mut RaylibCamera3D,
        u32::from(mode) as c_int,
      )
    }
  }
}
