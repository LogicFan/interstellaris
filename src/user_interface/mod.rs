use bevy::prelude::*;

mod cursor;
mod main_camera;
mod parsed_input;
mod settings;

pub use cursor::lock_cursor;
pub use cursor::release_cursor;
pub use main_camera::spawn_main_camera;
pub use main_camera::PrimaryCamera;
pub use main_camera::PrimaryCameraSystemSet;

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(settings::InputSettings::default())
            .add_systems(Startup, main_camera::spawn_main_camera)
            .add_systems(
                PostUpdate,
                (main_camera::move_main_camera, main_camera::zoom_main_camera)
                    .in_set(PrimaryCameraSystemSet),
            );
    }
}
