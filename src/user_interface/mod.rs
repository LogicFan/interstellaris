use bevy::prelude::*;

mod cursor;
mod parsed_input;
mod primary_camera;
mod settings;

use bevy::transform::TransformSystem;
pub use cursor::lock_cursor;
pub use cursor::release_cursor;
pub use primary_camera::spawn_primary_camera;
pub use primary_camera::PrimaryCamera;
pub use primary_camera::PrimaryCameraSystemSet;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(settings::InputSettings::default())
            .insert_resource(primary_camera::PrimaryCameraMotionMode::MenuScene)
            .add_systems(Startup, primary_camera::spawn_primary_camera)
            .add_systems(
                Update,
                (
                    primary_camera::free_motion::move_main_camera,
                    primary_camera::free_motion::zoom_main_camera,
                    primary_camera::free_motion::rotate_main_camera,
                )
                    .run_if(primary_camera::free_motion::is_free_motion)
                    .in_set(PrimaryCameraSystemSet),
            );
    }
}
