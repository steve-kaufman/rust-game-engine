#[derive(Debug, PartialEq)]
pub enum Error {
  MissingField(String),
  FieldNotAllowed(String),
  BadJSON,
}
