mod background;
mod primary_menu;
mod secondary_menu;

pub use crate::user_interface::camera::PrimaryCamera as UiCamera;
pub use crate::user_interface::settings::UiSettings;

use crate::utils::*;
use background::*;
use bevy::prelude::*;
use primary_menu::*;
use secondary_menu::*;
// use new_game_menu::*;

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app
            // menu background
            .add_systems(OnEnter(AppState::MenuScene), spawn_menu_background)
            .add_systems(
                OnExit(AppState::MenuScene),
                despawn_entities::<MenuBackground>,
            )
            // main menu
            .add_systems(OnEnter(MenuState::PrimaryMenu), spawn_main_menu)
            .add_systems(
                OnExit(MenuState::PrimaryMenu),
                despawn_entities::<PrimaryMenu>,
            )
            .add_systems(
                Update,
                primary_menu_button_handler.run_if(in_state(MenuState::PrimaryMenu)),
            )
            // new game menu
            .add_systems(OnEnter(MenuState::NewGameMenu), spawn_new_game_menu)
            .add_systems(
                OnExit(MenuState::NewGameMenu),
                despawn_entities::<NewGameMenu>,
            )
            // load game menu
            .add_systems(OnEnter(MenuState::LoadGameMenu), spawn_load_game_menu)
            .add_systems(
                OnExit(MenuState::LoadGameMenu),
                despawn_entities::<LoadGameMenu>,
            )
            // online menu
            .add_systems(OnEnter(MenuState::OnlineMenu), spawn_online_menu)
            .add_systems(
                OnExit(MenuState::OnlineMenu),
                despawn_entities::<OnlineMenu>,
            )
            // settings menu
            .add_systems(OnEnter(MenuState::SettingsMenu), spawn_settings_menu)
            .add_systems(
                OnExit(MenuState::SettingsMenu),
                despawn_entities::<LoadGameMenu>,
            );
    }
}

pub fn despawn_entities<T>(mut commands: Commands, q_entity: Query<Entity, With<T>>)
where
    T: Component,
{
    let entity = q_entity.single();
    commands.entity(entity).despawn_recursive();
}
