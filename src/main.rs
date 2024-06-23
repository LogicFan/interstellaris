use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use interstellaris::planetary_system::init_planetary_system;
use interstellaris::common::camera::setup_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, init_planetary_system)
        .add_systems(Startup, setup_camera)
        .run();
}
