// use game_engine_test::{Game, Transform};
// use std::fs;
// use std::str;

// #[test]
// fn is_empty_by_default() {
//   let game = Game::new();
//   assert_eq!(game.transforms(), []);
// }

// #[test]
// fn takes_one_transform() {
//   let game_state: Vec<u8> = fs::read("tests/good_game_states/one_entity.json").unwrap();
//   let game_state = str::from_utf8(game_state.as_slice()).unwrap();

//   let game = Game::from_state(game_state).unwrap();

//   assert_eq!(
//     game.transforms(),
//     [Transform {
//       x: 0.0,
//       y: -1.0,
//       w: 1.0,
//       h: 1.8
//     }]
//   )
// }

// #[test]
// fn errors_with_non_number_transform_values() {
//   let game_state: Vec<u8> = fs::read("tests/bad_game_states/bad_transform.json").unwrap();
//   let game_state = str::from_utf8(game_state.as_slice()).unwrap();

//   Game::from_state(game_state).unwrap_err(); // panics if there is no err
// }

// #[test]
// fn takes_multiple_transforms() {
//   let game_state: Vec<u8> = fs::read("tests/good_game_states/multi_entity.json").unwrap();
//   let game_state = str::from_utf8(game_state.as_slice()).unwrap();

//   let game = Game::from_state(game_state).unwrap();

//   assert_eq!(
//     game.transforms(),
//     [
//       Transform {
//         x: 0.0,
//         y: -1.0,
//         w: 1.0,
//         h: 1.8
//       },
//       Transform {
//         x: 2.5,
//         y: 0.5,
//         w: 0.5,
//         h: 0.5
//       },
//       Transform {
//         x: -0.5,
//         y: 0.5,
//         w: 0.5,
//         h: 0.5
//       }
//     ]
//   )
// }
