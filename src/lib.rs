#![allow(mixed_script_confusables)]

pub mod fleet;
pub mod game_map;
pub mod planetary_system;
pub mod ui;
pub mod utils;

use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_systems(PostStartup, complete_setup);
    }
}

/// The high-level state of the app
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Setup,
    InMenu,
    // TODO: make this into the sub-state of InMenu
    Loading,
    InGame,
}

pub fn complete_setup(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InMenu);
}
