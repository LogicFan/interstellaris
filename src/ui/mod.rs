//! UI-related codes, including menu UI, game UI and camera setup.

pub mod camera;
mod menu_ui;
mod mouse;
mod settings;

use bevy::prelude::*;
pub use camera::setup;
pub use camera::CameraMotionSystemSet;
pub use camera::PrimaryCamera;

pub use mouse::{confine_pointer, release_pointer};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(settings::SettingsPlugin)
            .add_plugins(menu_ui::InMenuPlugin)
            .add_plugins(camera::primary_camera::PrimaryCameraPlugin)
            .add_plugins(mouse::MousePointerPlugin);
    }
}
