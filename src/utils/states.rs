use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MyAppState {
    Menu,
    InGame,
    Paused,
}