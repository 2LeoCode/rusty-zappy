use {
  crate::raylib::bindings::{
    GenMeshCube, GenMeshCylinder, GenMeshKnot, GenMeshPlane, Mesh as RaylibMesh,
  },
  std::ffi::c_int,
};

pub struct Mesh {
  pub(crate) raylib_mesh: RaylibMesh,
}

impl Mesh {
  pub fn gen_cylinder(radius: f32, height: f32, slices: i32) -> Self {
    Self {
      raylib_mesh: unsafe { GenMeshCylinder(radius, height, slices as c_int) },
    }
  }

  pub fn gen_knot(radius: f32, size: f32, radSeg: i32, sides: i32) -> Self {
    Self {
      raylib_mesh: unsafe { GenMeshKnot(radius, size, radSeg as c_int, sides as c_int) },
    }
  }

  pub fn gen_cube(width: f32, height: f32, length: f32) -> Self {
    Self {
      raylib_mesh: unsafe { GenMeshCube(width, height, length) },
    }
  }

  pub fn gen_plane(width: f32, length: f32, res_x: i32, res_z: i32) -> Self {
    Self {
      raylib_mesh: unsafe { GenMeshPlane(width, length, res_x as c_int, res_z as c_int) },
    }
  }
}
