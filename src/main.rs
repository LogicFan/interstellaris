use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use sickle_ui::SickleUiPlugin;
use stellaris::game_map::gen::GampMapGenPlugin;
use stellaris::ui::*;
use stellaris::*;

fn main() {
    let mut app = App::new();

    // third-party plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(SickleUiPlugin);

    // debug plugins, TODO: remove before release
    // app.add_plugins(EditorPlugin::default());

    // internal plugins
    app.add_plugins(CorePlugin)
        .add_plugins(UserInterfacePlugin)
        .add_plugins(GampMapGenPlugin);

    app.run();
}
