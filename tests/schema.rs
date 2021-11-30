use game_engine_test::{parser, Game};

#[test]
fn top_level() {
  struct Test {
    input: &'static str,
    err: parser::Error,
  }

  let bad_game_states = &[
    Test {
      input: r"{}",
      err: parser::Error::MissingField("entities".to_string()),
    },
    Test {
      input: r#"{"foo": "bar"}"#,
      err: parser::Error::FieldNotAllowed("foo".to_string()),
    },
    Test {
      input: r#"{"bar": "foo"}"#,
      err: parser::Error::FieldNotAllowed("bar".to_string()),
    },
    Test {
      input: r#"{"Entities": [] }"#,
      err: parser::Error::FieldNotAllowed("Entities".to_string()),
    },
  ];

  for bad_state in bad_game_states {
    let err = Game::from_state(bad_state.input).unwrap_err();

    assert_eq!(bad_state.err, err, "Input: {}", bad_state.input)
  }
}
