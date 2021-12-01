use super::Error;
use jsonschema::error::ValidationErrorKind;
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default)]
pub struct GameState {
  entities: Vec<i32>,
}

pub fn parse_game_state(json_state: &str) -> Result<GameState, Error> {
  validate_with_jsonschema(json_state)?;

  match serde_json::from_str(json_state) {
    Ok(game_state) => Ok(game_state),
    Err(_) => Err(Error::BadJSON),
  }
}

fn validate_with_jsonschema(json_state: &str) -> Result<(), Error> {
  let schema = serde_json::to_value(json!({"entities": [1, 2]})).expect("A serializable");
  let validator: JSONSchema = JSONSchema::compile(&schema).expect("A valid schema");

  assert!(jsonschema::is_valid(
    &json!({"foo": "bar"}),
    &json!({"bar": "foo"})
  ));

  println!("Schema: '{}'", schema);

  let state_as_json_value = serde_json::from_str(json_state).or(Err(Error::BadJSON))?;

  println!("Input: '{}'", state_as_json_value);

  validate(&validator, &state_as_json_value)
}

fn validate(validator: &JSONSchema, state_as_json_value: &serde_json::Value) -> Result<(), Error> {
  match validator.validate(state_as_json_value) {
    Ok(_) => {
      println!("no errors");
      Ok(())
    }
    Err(errors) => {
      println!("errors");
      let errors: Vec<jsonschema::ValidationError> = errors.collect();
      let error = &errors[0];
      Err(map_error(error))
    }
  }
}

fn map_error(error: &jsonschema::ValidationError) -> Error {
  match &error.kind {
    ValidationErrorKind::AdditionalProperties { unexpected } => {
      Error::FieldNotAllowed(unexpected[0].clone())
    }
    ValidationErrorKind::Required { property } => Error::MissingField(property.to_string()),
    ValidationErrorKind::Type { kind } => Error::BadType {
      field: error.schema_path.to_string(),
      should_be: format!("{:?}", kind),
    },
    _ => Error::BadJSON,
  }
}
