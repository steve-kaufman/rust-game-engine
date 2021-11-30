use crate::parser;

use crate::parser::parse_game_state;

#[derive(Debug, PartialEq)]
pub enum Error {
  ParseError,
}

#[derive(Default, Debug)]
pub struct Game {}

impl Game {
  pub fn new() -> Game {
    Game {}
  }

  pub fn from_state(state: &str) -> Result<Game, parser::Error> {
    parse_game_state(state)?;
    Ok(Game {})
  }
}
