use rand::Rng;

use crate::{utils::Random, zappy::Item};

#[derive(Default, Clone)]
pub struct Tile {
  content: Option<Item>,
}

impl Tile {
  pub fn has_item(&self) -> bool {
    self.content.is_some()
  }

  pub fn fill_randomly(&mut self, rng: &mut impl Rng) {
    self.content = Some(Item::random(rng));
  }

  pub fn content(&self) -> &Option<Item> {
    &self.content
  }
}
