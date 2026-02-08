mod raylib;

use raylib::wrapper::{Color, get_current_monitor, init_window};

use crate::raylib::wrapper::{
  Camera3D, Camera3DMode, Camera3DProjection, KeyboardKey, Vector3, is_key_down,
};

fn main() {
  const WIDTH: i32 = 800;
  const HEIGHT: i32 = 600;
  let window = init_window(WIDTH, HEIGHT, "zappy").expect("Invalid arguments to init_window");
  let current_monitor = get_current_monitor();
  let cube_position = Vector3 {
    x: 0.,
    y: 0.,
    z: 0.,
  };

  let mut camera = Camera3D {
    position: Vector3 {
      x: 10.,
      y: 10.,
      z: 10.,
    },
    target: Vector3 {
      x: 0.,
      y: 0.,
      z: 0.,
    },
    up: Vector3 {
      x: 0.,
      y: 1.,
      z: 0.,
    },
    fovy: 25.,
    projection: Camera3DProjection::Orthographic,
  };
  let mut camera_speed = 1.;

  window.disable_cursor();
  window.set_target_fps(current_monitor.get_refresh_rate());
  while !window.should_close() {
    if is_key_down(KeyboardKey::W) {
      camera.move_up(camera_speed);
    }
    if is_key_down(KeyboardKey::S) {
      camera.move_up(-camera_speed);
    }
    if is_key_down(KeyboardKey::D) {
      camera.move_right(camera_speed, false);
    }
    if is_key_down(KeyboardKey::A) {
      camera.move_right(-camera_speed, false);
    }
    window.begin_drawing(|pen| {
      pen.clear_background(Color::RayWhite);
      pen.begin_mode_3d(&camera, |pen_3d| {
        pen_3d.draw_grid(10, 1.);
        pen_3d.draw_cube(cube_position, 2., 2., 2., Color::Red);
        pen_3d.draw_cube_wires(cube_position, 2., 2., 2., Color::Blue);
      });

      pen
        .draw_text("Urmom", WIDTH / 64, 0, HEIGHT / 8, Color::Blue)
        .expect("Invalid arguments to draw_text");
      println!("{:?}", camera);
    });
  }
}
