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
// use new_game_menu::*;

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
            .add_systems(OnEnter(MenuState::NewGameMenu), spawn_new_game_menu)
            .add_systems(
                OnExit(MenuState::NewGameMenu),
                despawn_entity::<NewGameMenu>,
            )
            // load game menu
            .add_systems(OnEnter(MenuState::LoadGameMenu), spawn_load_game_menu)
            .add_systems(
                OnExit(MenuState::LoadGameMenu),
                despawn_entity::<LoadGameMenu>,
            )
            // online menu
            .add_systems(OnEnter(MenuState::OnlineMenu), spawn_online_menu)
            .add_systems(
                OnExit(MenuState::OnlineMenu),
                despawn_entity::<OnlineMenu>,
            )
            // settings menu
            .add_systems(OnEnter(MenuState::SettingsMenu), spawn_settings_menu)
            .add_systems(
                OnExit(MenuState::SettingsMenu),
                despawn_entity::<SettingsMenu>,
            );
    }
}

pub fn despawn_entity<T>(mut commands: Commands, q_entity: Query<Entity, With<T>>)
where
    T: Component,
{
    let entity = q_entity.single();
    commands.entity(entity).despawn_recursive();
}
