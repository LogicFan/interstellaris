//! a module for the start menu.

mod background;
mod main_menu;
mod sub_menu;
mod new_game_menu;
mod ui_builder_ext;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use crate::ui::settings::UiSettings;

use super::spawn_primary_camera;
use crate::AppState;
use background::*;
use bevy::prelude::*;
use main_menu::*;
use sub_menu::*;
use new_game_menu::*;

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
            // menu background
            .add_systems(
                OnEnter(AppState::InMenu),
                spawn_menu_background.after(spawn_primary_camera),
            )
            // main menu
            .add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)
            // all sub menus
            .add_systems(
                Update,
                sub_menu_button_handler.run_if(
                    in_state(MenuState::NewGame)
                        .or_else(in_state(MenuState::LoadGame))
                        .or_else(in_state(MenuState::OnlineGame))
                        .or_else(in_state(MenuState::Settings)),
                ),
            )
            // new game menu
            .add_systems(OnEnter(MenuState::NewGame), spawn_new_game_menu)
            // load game menu
            .add_systems(
                OnEnter(MenuState::LoadGame),
                load_game::spawn_load_game_menu,
            )
            // online menu
            .add_systems(
                OnEnter(MenuState::OnlineGame),
                online_game::spawn_online_menu,
            )
            // settings menu
            .add_systems(OnEnter(MenuState::Settings), settings::spawn_settings_menu);
    }
}
