use crate::zappy::{Error, Player, Result, constants::TEAM_SIZE};

pub struct Team {
  name: String,
  players: [Option<Player>; TEAM_SIZE],
}

impl Team {
  pub(crate) fn empty(name: impl Into<String>) -> Self {
    Team {
      name: name.into(),
      players: [(); TEAM_SIZE].map(|()| None),
    }
  }

  pub(crate) fn add_player(&mut self) -> Result<usize> {
    for (i, p) in self.players.iter_mut().enumerate() {
      if p.is_none() {
        *p = Some(Player::new());
        return Ok(i);
      }
    }
    Err(Error::TeamIsFull(self.name.to_string()))
  }

  pub(crate) fn remove_player(&mut self, id: usize) -> Result<()> {
    use Error::*;
    self
      .players
      .get_mut(id)
      .ok_or(PlayerOutOfBounds(self.name.to_string(), id))
      .and_then(|o| {
        if o.is_none() {
          Err(PlayerNotFound(self.name.to_string(), id))
        } else {
          *o = None;
          Ok(())
        }
      })
  }
}
