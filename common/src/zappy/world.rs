use {
  crate::zappy::{Error, Player, Result, Team, Tile},
  rand::{Rng, RngExt},
  std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
  },
};

pub struct World {
  x: usize,
  y: usize,
  tiles: Vec<Tile>,
  teams: HashMap<String, Team>,
  player_positions: HashMap<(usize, usize), LinkedList<(String, usize)>>,
}

impl World {
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

  pub fn iter_tiles(&self) -> impl Iterator<Item = ((usize, usize), &Tile)> {
    self
      .tiles
      .iter()
      .enumerate()
      .map(|(i, t)| ((i / self.x(), i % self.x()), t))
  }

  pub fn tile_at_pos(&self, x: usize, y: usize) -> &Tile {
    &self.tiles[y * self.x + x]
  }

  pub fn tile_at_pos_mut(&mut self, x: usize, y: usize) -> &mut Tile {
    &mut self.tiles[y * self.x + x]
  }

  pub fn tile_at_index(&self, i: usize) -> &Tile {
    &self.tiles[i]
  }

  pub fn tile_at_index_mut(&mut self, i: usize) -> &mut Tile {
    &mut self.tiles[i]
  }

  pub fn teams(&self) -> &HashMap<String, Team> {
    &self.teams
  }

  pub fn teams_mut(&mut self) -> &mut HashMap<String, Team> {
    &mut self.teams
  }

  pub fn players_at_pos(
    &self,
    x: usize,
    y: usize,
  ) -> Option<impl Iterator<Item = &(String, usize)>> {
    self.player_positions.get(&(x, y)).map(|v| v.iter())
  }

  pub fn add_team(&mut self, name: impl Into<String> + AsRef<str>) -> Result<&mut Self> {
    let name = name.into();
    if self.teams.get_mut(name.as_str()).is_some() {
      Err(Error::TeamExists(name.clone()))
    } else {
      self.teams.insert(name.clone(), Team::empty(name));
      Ok(self)
    }
  }

  pub fn remove_team(&mut self, name: impl Into<String> + AsRef<str>) -> Result<&mut Self> {
    self
      .teams
      .remove(name.as_ref())
      .ok_or(Error::TeamDoesntExist(name.into()))?;
    Ok(self)
  }

  pub fn add_player(&mut self, team_name: impl Into<String> + AsRef<str>) -> Result<usize> {
    use Error::*;
    let team_name = team_name.into();
    self
      .teams
      .get_mut(team_name.as_str())
      .map_or(Err(TeamDoesntExist(team_name.clone())), |t| {
        let players = self.player_positions.entry((0, 0)).or_default();
        let id = t.add_player()?;
        players.push_back((team_name, id));
        Ok(id)
      })
  }

  pub fn remove_player(&mut self, team_name: String, id: usize) -> Result<&mut Self> {
    use Error::*;
    self
      .teams
      .get_mut(&team_name)
      .map_or(Err(TeamDoesntExist(team_name.clone())), |t| {
        t.remove_player(id)
      })?;
    Ok(self)
  }

  pub fn iter_players(
    &self,
  ) -> impl Iterator<Item = ((usize, usize), impl Iterator<Item = &(String, usize)>)> {
    self
      .player_positions
      .iter()
      .map(|(pos, player_infos)| (*pos, player_infos.iter()))
  }

  pub fn get_player(&mut self, team_name: String, id: usize) -> Result<&Player> {
    unimplemented!()
  }

  pub fn get_player_mut(&mut self, team_name: String, id: usize) -> Result<&Player> {
    unimplemented!()
  }
}
