//! a module for the start menu.

mod background;
mod load_game_page;
mod main_page;
mod new_game_page;
mod online_game_page;
mod settings_page;
mod ui_builder_ext;

use std::time::Duration;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use crate::ui::settings::UiSettings;

use crate::AppState;
use background::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use load_game_page::*;
use main_page::*;
use new_game_page::*;
use online_game_page::*;
use settings_page::*;

#[derive(SubStates, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[source(AppState = AppState::InMenu)]
pub enum MenuState {
    #[default]
    MainPage,
    NewGamePage,
    LoadGamePage,
    SettingsPage,
    OnlineGamePage,
}

#[derive(Resource, Default, Clone)]
pub struct PrevPageStack(Vec<MenuState>);

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>()
            .insert_resource(PrevPageStack::default())
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
            .add_systems(OnEnter(MenuState::MainPage), spawn_main_menu)
            .add_systems(OnEnter(MenuState::NewGamePage), spawn_new_game_menu)
            .add_systems(OnEnter(MenuState::LoadGamePage), spawn_load_game_menu)
            .add_systems(OnEnter(MenuState::OnlineGamePage), spawn_online_menu)
            .add_systems(OnEnter(MenuState::SettingsPage), spawn_settings_menu);
    }
}
