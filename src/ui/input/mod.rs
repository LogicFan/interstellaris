//! Handle mouse and keyboard input
mod mouse;
mod parsed;

use bevy::prelude::*;
use bevy_mod_picking::picking_core::PickSet;
pub use parsed::ParsedMouseInput;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(parsed::ParsedMouseInput::default())
            .add_systems(Startup, mouse::setup)
            .add_systems(
                First,
                mouse::post_setup.run_if(run_once()).in_set(PickSet::Input),
            )
            .add_systems(First, (mouse::button, mouse::motion).in_set(PickSet::Input))
            .add_systems(Update, mouse::sync);
    }
}
