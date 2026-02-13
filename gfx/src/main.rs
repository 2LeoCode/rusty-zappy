mod raylib;

use {
  crate::raylib::{
    Vector3,
    cameras::{Camera3D, Camera3DProjection},
    colors::Color,
    input::{KeyboardKey, get_mouse_wheel_move_y, is_key_down, is_mouse_wheel_moving_y},
    window::Window,
  },
  common::zappy::{World, constants::TEAM_SIZE},
  rand::rng,
  std::error::Error,
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

struct Scene {}

impl Scene {
  fn new() -> Self {
    Self {}
  }
}

fn gfx() -> Result<(), impl Error> {
  const GRID_SIZE: u32 = 128;
  let window = Window::try_init(1920, 1080, "zappy").expect("Invalid arguments to init_window");
  let current_monitor = window.get_current_monitor();
  let mut rng = rng();
  let mut world = World::generate(&mut rng, 128, 128);

  world.add_team("Team 1")?;
  world.add_team("Team 2")?;
  world.add_team("Team 3")?;

  for _ in 0..TEAM_SIZE {
    world.add_player("Team 1")?;
    world.add_player("Team 2")?;
    world.add_player("Team 3")?;
  }
  // window.set_size(current_monitor.width(), current_monitor.height());
  // window.set_position(0, 0);
  // let window = window;

  let cube_position = Vector3 {
    x: 0.,
    y: 0.,
    z: 0.,
  };

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

  window.set_target_fps(current_monitor.get_refresh_rate());
  Ok::<(), common::zappy::Error>(while !window.should_close() {
    update_camera(&mut camera);
    window.begin_drawing(|pen| {
      use Color::*;
      pen.clear_background(RayWhite);
      pen.begin_mode_3d(&camera, |pen_3d| {
        pen_3d.draw_grid(GRID_SIZE as i32, 1.);
        pen_3d.draw_cube(cube_position, 2., 2., 2., Red);
        pen_3d.draw_cube_wires(cube_position, 2., 2., 2., Blue);
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

fn main() {}
