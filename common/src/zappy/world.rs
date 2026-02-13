use {
  crate::zappy::{Error, Player, Result, Team, Tile},
  rand::{Rng, RngExt},
  std::collections::{HashMap, LinkedList},
};

pub struct World<'a> {
  x: usize,
  y: usize,
  tiles: Vec<Tile>,
  teams: HashMap<String, Team>,
  player_positions: HashMap<(usize, usize), LinkedList<&'a mut Player>>,
}

impl<'a> World<'a> {
  pub fn empty(x: usize, y: usize) -> Self {
    Self {
      x,
      y,
      tiles: vec![Tile::default(); x * y],
      teams: HashMap::new(),
      player_positions: HashMap::new(),
    }
  }

  pub fn generate(rng: &mut impl Rng, x: usize, y: usize) -> Self {
    let mut instance = Self::empty(x, y);
    let ore_count = x * y / 20;

    let mut i = 0;
    while i < ore_count {
      let j = rng.random::<u64>() as usize % instance.tiles.len();
      if instance.tiles[j].has_item() {
        continue;
      }
      instance.tiles[j].fill_randomly(rng);
      i += 1;
    }

    instance
  }

  pub fn x(&self) -> usize {
    self.x
  }

  pub fn y(&self) -> usize {
    self.y
  }

  pub fn tile_at_pos(&'a self, x: usize, y: usize) -> &'a Tile {
    &self.tiles[y * self.x + x]
  }

  pub fn tile_at_pos_mut(&'a mut self, x: usize, y: usize) -> &'a mut Tile {
    &mut self.tiles[y * self.x + x]
  }

  pub fn tile_at_index(&'a self, i: usize) -> &'a Tile {
    &self.tiles[i]
  }

  pub fn tile_at_index_mut(&'a mut self, i: usize) -> &'a mut Tile {
    &mut self.tiles[i]
  }

  pub fn teams(&'a self) -> &'a HashMap<String, Team> {
    &self.teams
  }

  pub fn teams_mut(&'a mut self) -> &'a mut HashMap<String, Team> {
    &mut self.teams
  }

  pub fn players_at_pos<'b>(
    &'a self,
    x: usize,
    y: usize,
  ) -> Option<impl Iterator<Item = &'a Player>>
  where
    'a: 'b,
  {
    self
      .player_positions
      .get(&(x, y))
      .map(|v| v.iter().map(|v| &**v))
  }

  pub fn player_at_pos_mut(
    &'a mut self,
    x: usize,
    y: usize,
  ) -> Option<impl Iterator<Item = &'a mut Player>> {
    self
      .player_positions
      .get_mut(&(x, y))
      .map(|v| v.iter_mut().map(|v| &mut **v))
  }

  pub fn add_team(&'a mut self, name: impl Into<String> + AsRef<str>) -> Result<()> {
    let name = name.into();
    self
      .teams
      .get_mut(name.as_str())
      .map(|_| Err::<(), _>(Error::TeamExists(name.clone())))
      .transpose()
      .map(|_| {
        self.teams.insert(name.clone(), Team::empty(name));
      })
  }

  pub fn remove_team(&'a mut self, name: impl Into<String> + AsRef<str>) -> Result<()> {
    self
      .teams
      .remove(name.as_ref())
      .map(|_| ())
      .ok_or(Error::TeamDoesntExist(name.into()))
  }

  pub fn add_player(&'a mut self, team_name: impl Into<String> + AsRef<str>) -> Result<usize> {
    use Error::*;
    self
      .teams
      .get_mut(team_name.as_ref())
      .map_or(Err(TeamDoesntExist(team_name.into())), |t| {
        // TODO: add player to player_positions
        Ok(t.add_player()?)
      })
  }

  pub fn remove_player(&'a mut self, team_name: String, id: usize) -> Result<()> {
    use Error::*;
    self
      .teams
      .get_mut(&team_name)
      .map_or(Err(TeamDoesntExist(team_name.clone())), |t| {
        Ok(t.remove_player(id)?)
      })
  }
}
