use super::Error;
use json::JsonValue;

pub struct GameState {}

pub fn parse_game_state(state: &str) -> Result<GameState, Error> {
  let state = match json::parse(state) {
    Ok(value) => value,
    Err(_) => return Err(Error::BadJSON),
  };

  match check_entries(&state) {
    Some(err) => Err(err),
    None => Ok(GameState {}),
  }
}

fn check_entries(state: &JsonValue) -> Option<Error> {
  for (key, _) in state.entries() {
    if key != "entities" {
      return Some(Error::FieldNotAllowed(key.to_string()));
    }
  }
  if !state.has_key("entities") {
    return Some(Error::MissingField("entities".to_string()));
  }
  None
}
