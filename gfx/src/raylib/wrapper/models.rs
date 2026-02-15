use crate::raylib::{
  bindings::{LoadModelFromMesh, Model as RaylibModel},
  meshes::Mesh,
  raymath::Matrix,
};

pub struct Model {
  pub(crate) raylib_model: RaylibModel,
}

impl Model {
  pub fn from_mesh(mesh: Mesh) -> Self {
    Self {
      raylib_model: unsafe { LoadModelFromMesh(mesh.raylib_mesh) },
    }
  }

  pub fn set_transform(&mut self, matrix: Matrix) {
    self.raylib_model.transform = matrix.raylib_matrix;
  }
}
