use bevy::prelude::*;

/// The high-level state of the app
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    /// When the app is in the menu. 
    MenuScene,
    /// When the app is in the game play.
    GameScene,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    /// if `AppState` == `MenuScene` and is in menu state, the default menu. 
    MainMenu,
    /// if `AppState` == `GameScene` and game is running
    InGame,
    /// if `AppState` == `GameScene` and game is paused
    Paused,
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum GameState {
//     Running,
//     Paused,
//     /// if AppState is not MenuScene
//     Invalid,
// }
