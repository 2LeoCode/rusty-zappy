use {
  crate::zappy::{Item, Result},
  std::collections::HashMap,
};

pub struct Player {
  level: u8,
  inventory: HashMap<Item, usize>,
}

impl Player {
  pub fn new() -> Self {
    Player {
      level: 0,
      inventory: HashMap::new(),
    }
  }

  pub fn level_up(&mut self) -> Result<()> {
    Ok(())
  }
}
