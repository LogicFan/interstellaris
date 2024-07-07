//! a module for the start menu.

mod background;
mod main_menu;
mod sub_menu;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use crate::ui::settings::UiSettings;

use super::spawn_primary_camera;
use crate::AppState;
use background::*;
use bevy::prelude::*;
use main_menu::*;
use sub_menu::*;

#[derive(SubStates, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[source(AppState = AppState::InMenu)]
pub enum MenuState {
    #[default]
    MainMenu,
    NewGame,
    LoadGame,
    SettingsMenu,
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
            .add_systems(
                Update,
                main_menu_button_handler.run_if(in_state(MenuState::MainMenu)),
            )
            // all sub menus
            .add_systems(
                Update,
                sub_menu_button_handler.run_if(
                    in_state(MenuState::NewGame)
                        .or_else(in_state(MenuState::LoadGame))
                        .or_else(in_state(MenuState::OnlineGame))
                        .or_else(in_state(MenuState::SettingsMenu)),
                ),
            )
            // new game menu
            .add_systems(OnEnter(MenuState::NewGame), new_game::spawn_new_game_menu)
            .add_systems(
                Update,
                new_game::confirm_button_handler.run_if(in_state(MenuState::NewGame)),
            )
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
            .add_systems(
                OnEnter(MenuState::SettingsMenu),
                settings::spawn_settings_menu,
            );
    }
}
