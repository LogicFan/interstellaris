mod background;
mod main_menu;

use crate::utils::*;
use background::*;
use bevy::prelude::*;
use main_menu::*;

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // menu background
            .add_systems(OnEnter(AppState::MenuScene), spawn_menu_background)
            .add_systems(OnExit(AppState::MenuScene), despawn_menu_background)
            // main menu
            .add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(MenuState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                new_game_handler.run_if(in_state(MenuState::MainMenu)),
            )
            .add_systems(
                Update,
                load_game_handler.run_if(in_state(MenuState::MainMenu)),
            )
            .add_systems(
                Update,
                settings_handler.run_if(in_state(MenuState::MainMenu)),
            )
            .add_systems(Update, online_handler.run_if(in_state(MenuState::MainMenu)))
            .add_systems(Update, exit_handler.run_if(in_state(MenuState::MainMenu)));
    }
}
