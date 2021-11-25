use crate::Transform;

use crate::game_state_parser::parse_game_state;

#[derive(Debug, PartialEq)]
pub enum Error {
  ParseError,
}

#[derive(Debug)]
pub struct Game {
  transforms: Vec<Transform>,
}

impl Game {
  pub fn new() -> Game {
    Game {
      transforms: Vec::new(),
    }
  }

  pub fn from_state(state: String) -> Result<Game, Error> {
    let game_state = parse_game_state(state)?;
    Ok(Game {
      transforms: game_state.transforms,
    })
  }

  pub fn transforms(&self) -> Vec<Transform> {
    self.transforms.clone()
  }
}
