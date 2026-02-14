use crate::raylib::{
  bindings::{LoadModelFromMesh, Model as RaylibModel},
  meshes::Mesh,
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
}
