use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    MenuScene,
    GameScene,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Running,
    Paused,
}
