use crate::{Error, Transform};
use json::JsonValue;

pub struct GameState {
  pub transforms: Vec<Transform>,
}

pub fn parse_game_state(state: String) -> Result<GameState, Error> {
  let state = json::parse(state.as_str()).unwrap();

  let entities = state["entities"].members();

  let transforms = match entities.map(entity_to_transform).collect() {
    Ok(transforms) => transforms,
    Err(e) => return Err(e),
  };

  Ok(GameState { transforms })
}

fn entity_to_transform(entity: &JsonValue) -> Result<Transform, Error> {
  let transform = &entity["transform"];

  let x = parse_f64(&transform["x"])?;
  let y = parse_f64(&transform["y"])?;
  let w = parse_f64(&transform["w"])?;
  let h = parse_f64(&transform["h"])?;

  Ok(Transform { x, y, w, h })
}

fn parse_f64(value: &JsonValue) -> Result<f64, Error> {
  match value.as_f64() {
    Some(value) => Ok(value),
    None => return Err(Error::ParseError),
  }
}
