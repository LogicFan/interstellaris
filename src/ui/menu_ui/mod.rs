//! a module for the start menu.

mod background;
mod load_game_menu;
mod main_menu;
mod new_game_menu;
mod online_game_menu;
mod settings_menu;
mod ui_builder_ext;

use std::time::Duration;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use crate::ui::settings::UiSettings;

use crate::AppState;
use background::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use load_game_menu::*;
use main_menu::*;
use new_game_menu::*;
use online_game_menu::*;
use settings_menu::*;

#[derive(SubStates, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[source(AppState = AppState::InMenu)]
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
                OnEnter(AppState::InMenu),
                (load_background, spawn_background).chain(),
            )
            .add_systems(OnExit(AppState::Loading), clean_background)
            .add_systems(
                Update,
                update_background.run_if(
                    on_timer(Duration::from_secs(10)).and_then(in_state(AppState::Loading)),
                ),
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
