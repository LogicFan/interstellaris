//! a module for the start menu.

mod background;
mod load_game_menu;
mod main_menu;
mod new_game_menu;
mod online_game_menu;
mod settings_menu;
mod ui_builder_ext;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use crate::ui::settings::UiSettings;

use super::spawn_primary_camera;
use crate::AppState;
use background::*;
use bevy::prelude::*;
use load_game_menu::*;
use main_menu::*;
use new_game_menu::*;
use online_game_menu::*;
use settings_menu::*;

#[derive(SubStates, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[source(AppState = AppState::MainMenu)]
pub enum MenuState {
    #[default]
    MainMenu,
    NewGame,
    LoadGame,
    Settings,
    OnlineGame,
}

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>()
            .add_systems(
                OnEnter(AppState::MainMenu),
                spawn_menu_background.after(spawn_primary_camera),
            )
            .add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)
            .add_systems(OnEnter(MenuState::NewGame), spawn_new_game_menu)
            .add_systems(OnEnter(MenuState::LoadGame), spawn_load_game_menu)
            // online menu
            .add_systems(OnEnter(MenuState::OnlineGame), spawn_online_menu)
            // settings menu
            .add_systems(OnEnter(MenuState::Settings), spawn_settings_menu);
    }
}
