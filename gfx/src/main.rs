mod raylib;

use std::thread::current;

use raylib::wrapper::{Color, get_current_monitor, init_window};

use crate::raylib::wrapper::{
  Camera3D, Camera3DMode, Camera3DProjection, KeyboardKey, Vector3, get_mouse_wheel_move_y,
  is_key_down, is_mouse_wheel_moving_y,
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

fn main() {
  const GRID_SIZE: u32 = 128;
  let window = init_window(1920, 1080, "zappy").expect("Invalid arguments to init_window");
  let current_monitor = get_current_monitor();
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
  while !window.should_close() {
    update_camera(&mut camera);
    window.begin_drawing(|pen| {
      pen.clear_background(Color::RayWhite);
      pen.begin_mode_3d(&camera, |pen_3d| {
        pen_3d.draw_grid(GRID_SIZE as i32, 1.);
        pen_3d.draw_cube(cube_position, 2., 2., 2., Color::Red);
        pen_3d.draw_cube_wires(cube_position, 2., 2., 2., Color::Blue);
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
  }
}
