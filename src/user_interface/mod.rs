use crate::utils::AppState;
use bevy::prelude::*;
use menu::{despawn_menu_background, spawn_menu_background};

mod camera;
mod cursor;
mod menu;
mod settings;

pub use camera::spawn_primary_camera;
pub use camera::CameraMotionSystemSet;
pub use camera::PrimaryCamera;
pub use cursor::lock_cursor;
pub use cursor::release_cursor;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(settings::InputSettings::default())
            .insert_resource(camera::MotionMode::NoMotion)
            .add_systems(Startup, camera::spawn_primary_camera)
            .add_systems(
                Update,
                (
                    camera::move_main_camera,
                    camera::zoom_main_camera,
                    camera::rotate_main_camera,
                )
                    .run_if(camera::is_free_motion)
                    .in_set(CameraMotionSystemSet::PrimaryCamera),
            )
            .add_systems(OnEnter(AppState::MenuScene), spawn_menu_background)
            .add_systems(OnExit(AppState::MenuScene), despawn_menu_background);
    }
}
