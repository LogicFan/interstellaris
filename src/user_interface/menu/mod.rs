mod background;
mod new_game_menu;
mod primary_menu;

use crate::utils::*;
use background::*;
use bevy::prelude::*;
use primary_menu::*;
// use new_game_menu::*;

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
                primary_menu_button_handler.run_if(in_state(MenuState::MainMenu)),
            );
        // new game menu
        // .add_systems(OnEnter(MenuState::NewGameMenu), spawn_new_game_menu)
        // .add_systems(OnExit(MenuState::NewGameMenu), despawn_new_game_menu);
    }
}
