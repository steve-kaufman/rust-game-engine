#[derive(Debug, PartialEq)]
pub enum Error {
  MissingField(String),
  FieldNotAllowed(String),
  BadJSON,
  BadType { field: String, should_be: String },
}
