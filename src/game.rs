use crate::Transform;
use json;

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
    let state = json::parse(state.as_str()).unwrap();

    let entities = state["entities"].members();

    let transforms = entities.map(|entity| -> Result<Transform, Error> {
      let transform = &entity["transform"];
      let x = match (&transform["x"]).as_f64() {
        Some(x) => x,
        None => return Err(Error::ParseError),
      };
      let y = match (&transform["y"]).as_f64() {
        Some(y) => y,
        None => return Err(Error::ParseError),
      };
      let w = match (&transform["w"]).as_f64() {
        Some(w) => w,
        None => return Err(Error::ParseError),
      };
      let h = match (&transform["h"]).as_f64() {
        Some(h) => h,
        None => return Err(Error::ParseError),
      };
      Ok(Transform { x, y, w, h })
    });

    let transforms = match transforms.collect() {
      Ok(transforms) => transforms,
      Err(e) => return Err(e),
    };

    Ok(Game { transforms })
  }

  pub fn transforms(&self) -> Vec<Transform> {
    self.transforms.clone()
  }
}
