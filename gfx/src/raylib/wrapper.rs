use {
  super::bindings::InitWindow,
  std::ffi::{CString, NulError},
  thiserror::Error,
};

#[derive(Error, Debug)]
pub enum InitWindowError {
  #[error("nul character in title")]
  TitleContainsNul(#[from] NulError),
}

pub struct Window;

pub fn init_window(width: i32, height: i32, title: &str) -> Result<Window, InitWindowError> {
  unsafe {
    InitWindow(width, height, CString::new(title)?.as_ptr());
  }
  Ok(Window {})
}
