//! The menu UI.

mod configs;
// mod load_game_page;
mod main_page;
// mod new_game_page;
// mod online_game_page;
// mod settings_page;
// mod menu_page;
mod menu_pages;
mod wallpaper;

pub use crate::ui::camera::PrimaryCamera as UiCamera;
pub use configs::Configs as UiConfigs;

use crate::states::AppStateLoading;
use crate::AppState;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
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

/// the plugin for setup and manage menu.
pub struct InMenuPlugin;

impl Plugin for InMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
            .enable_state_scoped_entities::<MenuState>()
            .insert_resource(configs::Configs::default())
            .add_systems(OnEnter(AppState::InMenu), wallpaper::setup)
            .add_systems(OnExit(AppStateLoading), wallpaper::cleanup)
            .add_systems(
                Update,
                wallpaper::update
                    .run_if(on_timer(Duration::from_secs(10)).and_then(in_state(AppStateLoading))),
            )
            .add_systems(OnEnter(MenuState::MainPage), main_page::setup)
            .add_systems(OnEnter(MenuState::NewGamePage), menu_pages::setup_new_game_page);
            // .add_systems(OnEnter(MenuState::LoadGamePage), spawn_load_game_menu)
            // .add_systems(OnEnter(MenuState::OnlineGamePage), spawn_online_menu)
            // .add_systems(OnEnter(MenuState::SettingsPage), spawn_settings_menu);
    }
}
