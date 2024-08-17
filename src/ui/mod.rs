//! UI-related codes, including menu UI, game UI and camera setup.

pub mod camera;
mod menu_ui;
mod settings;
mod input;

use bevy::prelude::*;
pub use camera::setup;
pub use camera::PrimaryCamera;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(settings::SettingsPlugin)
            .add_plugins(menu_ui::InMenuPlugin)
            .add_plugins(input::InputPlugin)
            .add_plugins(camera::primary_camera::PrimaryCameraPlugin);
    }
}
