use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use stellaris::planetary_system::init_planetary_system;
use stellaris::user_interface::*;
use bevy_mod_picking::DefaultPickingPlugins;
use stellaris::utils::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .insert_state(AppState::MenuScene)
        .add_systems(Startup, init_planetary_system)
        .add_systems(Startup, lock_cursor)
        .add_plugins(UserInterfacePlugin)
        .run();
}
