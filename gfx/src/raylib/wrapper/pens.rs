use {
  crate::raylib::{
    bindings::{
      ClearBackground, DEG2RAD, DrawCube, DrawCubeWires, DrawGrid, DrawModel, DrawText, EndDrawing,
      EndMode3D, Model as RaylibModel, RL_MODELVIEW, RL_PROJECTION, Vector3,
      rlDrawRenderBatchActive, rlEnableDepthTest, rlFrustum, rlGetCullDistanceNear, rlLoadIdentity,
      rlMatrixMode, rlMultMatrixf, rlOrtho, rlPushMatrix,
    },
    custom_bindings::{MatrixLookAt, MatrixToFloatV},
    wrapper::{
      cameras::{Camera3D, Camera3DProjection},
      colors::Color,
    },
  },
  std::ffi::{CString, NulError, c_int},
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum DrawTextError {
  #[error("nul character in text")]
  TextContainsNul(#[from] NulError),
}

pub struct Pen {
  pub(crate) __: (),
}

pub struct Pen3D {
  pub(crate) __: (),
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
    pos_x: u32,
    pos_y: u32,
    font_size: u32,
    color: Color,
  ) -> Result<(), DrawTextError> {
    let text = CString::new(text)?;

    unsafe {
      DrawText(
        text.as_ptr(),
        pos_x as c_int,
        pos_y as c_int,
        font_size as c_int,
        color.into(),
      )
    }
    Ok(())
  }

  pub fn begin_mode_3d<F: FnOnce(Pen3D)>(&self, camera: &Camera3D, callback: F) {
    use Camera3DProjection::*;
    unsafe {
      rlDrawRenderBatchActive();
      rlMatrixMode(RL_PROJECTION as c_int);
      rlPushMatrix();
      rlLoadIdentity();

      let aspect = camera.window.width as f64 / camera.window.height as f64;

      match camera.projection {
        Perspective => {
          let top =
            rlGetCullDistanceNear() * (camera.fovy() as f64 * 0.5f64 * DEG2RAD).tan() as f64;
          let right = top * aspect;

          rlFrustum(-right, right, -top, top, camera.near, camera.far)
        }
        Orthographic => {
          let top = (camera.fovy() / 2.) as f64;
          let right = top * aspect;

          rlOrtho(-right, right, -top, top, camera.near, camera.far);
        }
      }

      rlMatrixMode(RL_MODELVIEW as c_int);
      rlLoadIdentity();

      let mat_view = MatrixLookAt(
        camera.raylib_camera_3d.position,
        camera.raylib_camera_3d.target,
        camera.raylib_camera_3d.up,
      );
      rlMultMatrixf(MatrixToFloatV(mat_view).v.as_ptr());
      rlEnableDepthTest();
    }
    callback(Pen3D { __: () });
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
