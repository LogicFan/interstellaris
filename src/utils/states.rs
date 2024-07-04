use bevy::prelude::*;

/// The high-level state of the app
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MenuScene,
    GameScene,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    // if `AppState` == `MenuScene`
    PrimaryMenu,
    NewGameMenu,
    LoadGameMenu,
    SettingsMenu,
    OnlineMenu,
    // if `AppState` == `GameScene`
    InGame,
    Paused,
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum GameState {
//     Running,
//     Paused,
//     /// if AppState is not MenuScene
//     Invalid,
// }
