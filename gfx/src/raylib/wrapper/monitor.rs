use {
  crate::raylib::bindings::{GetMonitorHeight, GetMonitorRefreshRate, GetMonitorWidth},
  std::ffi::c_int,
};

#[derive(Debug)]
pub struct Monitor {
  pub(crate) id: i32,
}

impl Monitor {
  pub fn get_refresh_rate(&self) -> i32 {
    (unsafe { GetMonitorRefreshRate(self.id as c_int) }) as i32
  }

  pub fn width(&self) -> u32 {
    (unsafe { GetMonitorWidth(self.id as c_int) }) as u32
  }

  pub fn height(&self) -> u32 {
    (unsafe { GetMonitorHeight(self.id as c_int) }) as u32
  }
}
