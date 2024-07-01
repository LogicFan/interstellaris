use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use sickle_ui::SickleUiPlugin;
use stellaris::planetary_system::init_planetary_system;
use stellaris::user_interface::*;
use stellaris::utils::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(SickleUiPlugin)
        // TODO: remove before release
        .add_plugins(EditorPlugin::default())
        .insert_state(AppState::MenuScene)
        .add_plugins(UserInterfacePlugin)
        .add_systems(Startup, init_planetary_system)
        // .add_systems(Startup, lock_cursor)
        .run();
}
