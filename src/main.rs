use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use stellaris::planetary_system::init_planetary_system;
use stellaris::user_interface::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, init_planetary_system)
        .add_systems(PreStartup, init_main_camera)
        .add_systems(
            Startup,
            (spawn_virtual_pointer, enable_virtual_pointer).chain(),
        )
        .add_systems(
            Update,
            (
                update_virtual_pointer,
                sync_virtual_pointer.after(update_virtual_pointer),
            ),
        )
        .run();
}
