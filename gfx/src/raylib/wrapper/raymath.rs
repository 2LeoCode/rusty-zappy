use {
  crate::raylib::{bindings::Matrix as RaylibMatrix, custom_bindings::MatrixRotateY},
  std::ffi::c_float,
};

pub struct Matrix {
  pub(crate) raylib_matrix: RaylibMatrix,
}

impl Matrix {
  pub fn rotate_y(angle: f32) -> Self {
    Self {
      raylib_matrix: unsafe { MatrixRotateY(angle as c_float) },
    }
  }
}
