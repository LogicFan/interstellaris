//! UI-related codes, including menu UI, game UI and camera setup.

pub mod camera;
mod cursor;
mod menu_ui;
mod settings;

use bevy::prelude::*;
pub use camera::spawn_primary_camera;
pub use camera::CameraMotionSystemSet;
pub use camera::PrimaryCamera;
pub use cursor::lock_cursor;
pub use cursor::release_cursor;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(settings::SettingsPlugin)
            .add_plugins(menu_ui::InMenuPlugin)
            .add_plugins(camera::primary_camera::PrimaryCameraPlugin);
    }
}
