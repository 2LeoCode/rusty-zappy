mod raylib;

use {
  crate::raylib::{
    Vector3,
    cameras::{Camera3D, Camera3DProjection},
    colors::Color,
    input::{KeyboardKey, get_mouse_wheel_move_y, is_key_down, is_mouse_wheel_moving_y},
    meshes::Mesh,
    models::Model,
    pens::Pen3D,
    window::{Builder as WindowBuilder, ConfigFlag, Window},
  },
  common::{
    utils::Random,
    zappy::{Item, Ore, World, constants::TEAM_SIZE},
  },
  rand::{Rng, rng},
  std::{collections::HashMap, error::Error, process::ExitCode},
};

const CAMERA_SPEED: f32 = 1.;
const CAMERA_FOVY_MIN: f32 = 10.;
const CAMERA_FOVY_MAX: f32 = 280.;

fn update_camera(camera: &mut Camera3D) {
  if is_key_down(KeyboardKey::W) {
    camera.move_up(CAMERA_SPEED);
  }
  if is_key_down(KeyboardKey::S) {
    camera.move_up(-CAMERA_SPEED);
  }
  if is_key_down(KeyboardKey::D) {
    camera.move_right(CAMERA_SPEED, false);
  }
  if is_key_down(KeyboardKey::A) {
    camera.move_right(-CAMERA_SPEED, false);
  }
  if is_mouse_wheel_moving_y() {
    let new_fovy = (camera.fovy()
      + get_mouse_wheel_move_y() * -CAMERA_SPEED * camera.fovy() / CAMERA_FOVY_MIN)
      .round();
    camera.set_fovy(if new_fovy < CAMERA_FOVY_MIN {
      CAMERA_FOVY_MIN
    } else if new_fovy > CAMERA_FOVY_MAX {
      CAMERA_FOVY_MAX
    } else {
      new_fovy
    });
  }
}

struct Engine {
  scene: Scene,
  world: World,
}

struct Scene {
  plane: Model,
  nourritures: Vec<((f32, f32), Model, Color)>,
  ores: Vec<((f32, f32), Model, Color)>,
  players: Vec<((f32, f32), Model, Color)>,
}

impl Scene {
  fn world_to_scene_pos(world: &World, (x, y): (usize, usize)) -> (f32, f32) {
    (
      x as f32 - world.x() as f32 / 2. + 1.,
      y as f32 - world.y() as f32 / 2. + 1.,
    )
  }

  fn new(world: &World, rng: &mut impl Rng) -> Self {
    let mut ore_colors = HashMap::<Ore, Color>::new();
    Self {
      plane: Model::from_mesh(Mesh::gen_plane(world.x() as f32, world.y() as f32, 1, 1)),
      nourritures: world
        .iter_tiles()
        .filter_map(|(pos, t)| {
          t.content().as_ref().and_then(|item| match item {
            Item::Nourriture => Some((
              Self::world_to_scene_pos(world, pos),
              Model::from_mesh(Mesh::gen_knot(1., 0.3, 16, 64)),
              Color::Orange,
            )),
            Item::Ore(_) => None,
          })
        })
        .collect(),
      ores: world
        .iter_tiles()
        .filter_map(|(pos, t)| {
          t.content().as_ref().and_then(|item| match item {
            Item::Ore(ore) => Some((
              Self::world_to_scene_pos(world, pos),
              Model::from_mesh(Mesh::gen_cube(0.3, 0.3, 0.3)),
              ore_colors
                .entry(ore.clone())
                .or_insert(Color::random(rng))
                .clone(),
            )),
            Item::Nourriture => None,
          })
        })
        .collect(),
      players: vec![],
    }
  }

  fn render(&self, pen_3d: &Pen3D) {
    pen_3d.draw_grid(128, 1.);
    pen_3d.draw_model(
      &self.plane,
      Vector3 {
        x: 0.,
        y: -0.01,
        z: 0.,
      },
      1.,
      Color::Green,
    );
    for ((x, z), model, color) in self.nourritures.iter().chain(self.ores.iter()) {
      pen_3d.draw_model(
        model,
        Vector3 {
          x: *x,
          y: 0.7,
          z: *z,
        },
        1.,
        color.clone(),
      );
    }
  }
}

fn gfx() -> Result<(), impl Error> {
  use ConfigFlag::*;

  let mut window_builder = WindowBuilder::try_new("zappy").expect("null character in title");
  window_builder.add_config_flag(VsyncHint);
  let mut window = window_builder.build(800, 600);

  let current_monitor = window.get_current_monitor();
  window.set_size(current_monitor.width(), current_monitor.height());
  window.set_position(0, 0);

  let mut rng = rng();
  let mut world = World::generate(&mut rng, 127, 127);

  let scene = Scene::new(&world, &mut rng);

  world
    .add_team("Team 1")?
    .add_team("Team 2")?
    .add_team("Team 3")?;

  for _ in 0..TEAM_SIZE {
    world.add_player("Team 1")?;
    world.add_player("Team 2")?;
    world.add_player("Team 3")?;
  }

  // let window = window;

  // let cube_position = Vector3 {
  //   x: 0.,
  //   y: 0.,
  //   z: 0.,
  // };

  let mut camera = window.new_camera_3d(
    Vector3 {
      x: 0.,
      y: 0.,
      z: 0.,
    },
    Vector3 {
      x: -1.,
      y: -1.,
      z: -1.,
    },
    Vector3 {
      x: 0.,
      y: 1.,
      z: 0.,
    },
    CAMERA_FOVY_MIN,
    Camera3DProjection::Orthographic,
  );

  // window.set_target_fps(current_monitor.get_refresh_rate());
  Ok::<(), common::zappy::Error>(while !window.should_close() {
    update_camera(&mut camera);
    window.begin_drawing(|pen| {
      use Color::*;
      pen.clear_background(RayWhite);
      pen.begin_mode_3d(&camera, |pen_3d| {
        scene.render(&pen_3d);
        // pen_3d.draw_grid(GRID_SIZE as i32, 1.);
        // pen_3d.draw_cube(cube_position, 2., 2., 2., Red);
        // pen_3d.draw_cube_wires(cube_position, 2., 2., 2., Blue);
      });

      // pen
      //   .draw_text(
      //     "urmom",
      //     current_monitor.width() / 64u32,
      //     0,
      //     current_monitor.height() / 8u32,
      //     Color::Blue,
      //   )
      //   .expect("Invalid arguments to draw_text");
    });
  })
}

fn main() -> ExitCode {
  if let Err(err) = gfx() {
    println!("Error: {}", err);
    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
