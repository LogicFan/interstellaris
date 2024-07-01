use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MenuScene,
    GameScene,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuState {
    MainMenu,
    Invalid,
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// pub enum GameState {
//     Running,
//     Paused,
// }
