#[derive(Debug, PartialEq)]
pub enum Error {
  MissingField(String),
  FieldNotAllowed(String),
  BadJSON,
  // BadType {
  //   field: &'static str,
  //   must_be: &'static str,
  // },
}
