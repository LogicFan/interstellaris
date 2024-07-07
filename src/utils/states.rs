use bevy::prelude::*;

/// The high-level state of the app
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    MenuScene,
    LoadScene,
    GameScene,
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum GameState {
//     Running,
//     Paused,
//     /// if AppState is not MenuScene
//     Invalid,
// }
