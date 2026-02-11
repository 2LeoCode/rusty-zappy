use crate::raylib::bindings::{Matrix, Vector3, float16};

unsafe extern "C" {
  #[inline]
  pub fn MatrixLookAt(eye: Vector3, target: Vector3, up: Vector3) -> Matrix;

  #[inline]
  pub fn MatrixToFloatV(matrix: Matrix) -> float16;
}
