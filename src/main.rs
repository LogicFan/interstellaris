use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use sickle_ui::SickleUiPlugin;
use stellaris::game_map::GampMapGenPlugin;
use stellaris::ui::*;
use stellaris::utils::*;

fn main() {
    let mut app = App::new();

    // third-party plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(SickleUiPlugin);

    // debug plugins, TODO: remove before release
    // app.add_plugins(EditorPlugin::default());

    // internal plugins
    app.insert_state(AppState::MenuScene)
        .insert_state(MenuState::MainMenu)
        .add_plugins(GampMapGenPlugin)
        .add_plugins(UserInterfacePlugin);

    app.run();
}
