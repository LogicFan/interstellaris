//! The menu UI.

mod background;
mod load_game_page;
mod main_page;
mod new_game_page;
mod online_game_page;
mod settings_page;
mod ui_builder_ext;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use crate::ui::settings::UiSettings;

use crate::states::AppStateLoading;
use crate::AppState;
use background::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use load_game_page::*;
use main_page::*;
use new_game_page::*;
use online_game_page::*;
use settings_page::*;
use std::time::Duration;

/// The sub-state for menu page management.
#[derive(SubStates, Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[source(AppState = AppState::InMenu)]
pub enum MenuState {
    #[default]
    MainPage,
    NewGamePage,
    LoadGamePage,
    SettingsPage,
    OnlineGamePage,
}

/// A resource to keep track which page (i.e. state) we should
/// return to.
#[derive(Resource, Clone, Default)]
pub struct PrevPageStack(Vec<MenuState>);

/// the plugin for setup and manage menu.
pub struct InMenuPlugin;

impl Plugin for InMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>()
            .insert_resource(PrevPageStack::default())
            .add_systems(OnEnter(AppState::InMenu), spawn_background)
            .add_systems(OnExit(AppStateLoading), despawn_background)
            .add_systems(
                Update,
                update_background
                    .run_if(on_timer(Duration::from_secs(10)).and_then(in_state(AppStateLoading))),
            )
            .add_systems(OnEnter(MenuState::MainPage), spawn_main_menu)
            .add_systems(OnEnter(MenuState::NewGamePage), spawn_new_game_menu)
            .add_systems(OnEnter(MenuState::LoadGamePage), spawn_load_game_menu)
            .add_systems(OnEnter(MenuState::OnlineGamePage), spawn_online_menu)
            .add_systems(OnEnter(MenuState::SettingsPage), spawn_settings_menu);
    }
}
