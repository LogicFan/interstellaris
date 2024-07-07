//! a module for the start menu.

mod background;
mod main_menu;
mod sub_menu;

pub use crate::user_interface::camera::PrimaryCamera as UiCamera;
pub use crate::user_interface::settings::UiSettings;

use crate::utils::*;
use background::*;
use bevy::prelude::*;
use main_menu::*;
use sub_menu::*;

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // menu background
            .add_systems(OnEnter(AppState::MenuScene), spawn_menu_background)
            .add_systems(
                OnExit(AppState::MenuScene),
                despawn_entity::<MenuBackground>,
            )
            // main menu
            .add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(MenuState::MainMenu), despawn_entity::<MainMenu>)
            .add_systems(
                Update,
                main_menu_button_handler.run_if(in_state(MenuState::MainMenu)),
            )
            // all sub menus
            .add_systems(
                Update,
                sub_menu_button_handler.run_if(
                    in_state(MenuState::NewGameMenu)
                        .or_else(in_state(MenuState::LoadGameMenu))
                        .or_else(in_state(MenuState::OnlineMenu))
                        .or_else(in_state(MenuState::SettingsMenu)),
                ),
            )
            // new game menu
            .add_systems(
                OnEnter(MenuState::NewGameMenu),
                new_game::spawn_new_game_menu,
            )
            .add_systems(
                OnExit(MenuState::NewGameMenu),
                despawn_entity::<new_game::NewGameMenu>,
            )
            .add_systems(
                Update,
                new_game::confirm_button_handler.run_if(in_state(MenuState::NewGameMenu)),
            )
            // load game menu
            .add_systems(
                OnEnter(MenuState::LoadGameMenu),
                load_game::spawn_load_game_menu,
            )
            .add_systems(
                OnExit(MenuState::LoadGameMenu),
                despawn_entity::<load_game::LoadGameMenu>,
            )
            // online menu
            .add_systems(OnEnter(MenuState::OnlineMenu), online::spawn_online_menu)
            .add_systems(
                OnExit(MenuState::OnlineMenu),
                despawn_entity::<online::OnlineMenu>,
            )
            // settings menu
            .add_systems(
                OnEnter(MenuState::SettingsMenu),
                settings::spawn_settings_menu,
            )
            .add_systems(
                OnExit(MenuState::SettingsMenu),
                despawn_entity::<settings::SettingsMenu>,
            );
    }
}

/// despawn entity and its children.
/// TODO: use State Scoped Entities after Bevy 0.14
pub fn despawn_entity<T>(mut commands: Commands, q_entity: Query<Entity, With<T>>)
where
    T: Component,
{
    let entity = q_entity.single();
    commands.entity(entity).despawn_recursive();
}
