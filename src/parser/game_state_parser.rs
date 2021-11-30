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

  // let transforms = match entities.map(entity_to_transform).collect() {
  //   Ok(transforms) => transforms,
  //   Err(e) => return Err(e),
  // };

  // Ok(GameState { transforms })
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

// fn entity_to_transform(entity: &JsonValue) -> Result<Transform, Error> {
//   // let transform = &entity["transform"];

//   // let x = parse_f64(&transform["x"])?;
//   // let y = parse_f64(&transform["y"])?;
//   // let w = parse_f64(&transform["w"])?;
//   // let h = parse_f64(&transform["h"])?;

//   // Ok(Transform { x, y, w, h })
// }

// fn parse_f64(value: &JsonValue) -> Result<f64, Error> {
//   // match value.as_f64() {
//   //   Some(value) => Ok(value),
//   //   None => Err(Error::MissingField("foo")),
//   // }
// }
