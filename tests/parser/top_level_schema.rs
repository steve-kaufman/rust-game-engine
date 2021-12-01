use game_engine_test::{parser, Game};

/**
 * Allowed top-level schema:
 *
 * { "entities": [] }
 */

#[test]
fn it_rejects_bad_schema() {
  struct Test {
    json: &'static str,

    expected_err: parser::Error,
  }

  let bad_game_states = &[
    Test {
      json: r"{}",

      expected_err: parser::Error::BadJSON,
    },
    Test {
      json: r#"{ "foo": "bar" }"#,

      expected_err: parser::Error::FieldNotAllowed("foo".to_string()),
    },
    Test {
      json: r#"{ "bar": "foo" }"#,

      expected_err: parser::Error::FieldNotAllowed("bar".to_string()),
    },
    Test {
      json: r#"{ "Entities": [] }"#,

      expected_err: parser::Error::FieldNotAllowed("Entities".to_string()),
    },
    Test {
      json: r#"{ "entities": "foo" }"#,

      expected_err: parser::Error::BadType {
        field: "entities".to_string(),
        should_be: "array".to_string(),
      },
    },
  ];

  for bad_state in bad_game_states {
    let err = Game::from_state(bad_state.json).unwrap_err();

    assert_eq!(bad_state.expected_err, err, "Input: '{}'", bad_state.json)
  }
}
