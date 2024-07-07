use bevy::prelude::*;

/// The high-level state of the app
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    MenuScene,
    LoadScene,
    GameScene,
}

// TODO: change to sub-state after upgrade to Bevy 0.14
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    // if `AppState` == `MenuScene`
    MainMenu,
    NewGameMenu,
    LoadGameMenu,
    SettingsMenu,
    OnlineMenu,
    // if `AppState` == `LoadScene`
    Generating,
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
