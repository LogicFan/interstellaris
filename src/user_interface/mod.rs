use bevy::prelude::*;

mod cursor;
mod primary_camera;
mod parsed_input;
mod settings;

pub use cursor::lock_cursor;
pub use cursor::release_cursor;
pub use primary_camera::spawn_main_camera;
pub use primary_camera::PrimaryCamera;
pub use primary_camera::PrimaryCameraSystemSet;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(settings::InputSettings::default())
            .insert_resource(primary_camera::PrimaryCameraConstraint::default())
            .add_systems(Startup, primary_camera::spawn_main_camera)
            .add_systems(
                PostUpdate,
                (
                    primary_camera::move_main_camera,
                    primary_camera::zoom_main_camera,
                    primary_camera::rotate_main_camera,
                )
                    .in_set(PrimaryCameraSystemSet),
            );
    }
}
