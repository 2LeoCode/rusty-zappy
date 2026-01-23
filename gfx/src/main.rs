mod raylib;
use raylib::wrapper::{RaylibColor, init_window};

fn main() {
  let window = init_window(800, 600, "urmom").expect("Failed to create window");

  window.set_target_fps(60);

  while !window.should_close() {
    let drawer = window.begin_drawing();
    drawer.clear_background(RaylibColor::Red);
  }
}
