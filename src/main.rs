use bevy::prelude::*;
use bevy_mod_picking::{low_latency_window_plugin, picking_core, DefaultPickingPlugins};
use sickle_ui::SickleUiPlugin;
use stellaris::game_map::gen::GampMapGenPlugin;
use stellaris::ui::*;
use stellaris::*;

fn main() {
    let mut app = App::new();

    // third-party plugins
    app.add_plugins(DefaultPlugins.set(low_latency_window_plugin()))
        .add_plugins(picking_core::CorePlugin)
        .add_plugins(picking_core::InteractionPlugin)
        .add_plugins(bevy_picking_ui::BevyUiBackend)
        .add_plugins(SickleUiPlugin);

    // internal plugins
    app.add_plugins(CorePlugin)
        .add_plugins(UserInterfacePlugin)
        .add_plugins(GampMapGenPlugin);

    app.run();
}
