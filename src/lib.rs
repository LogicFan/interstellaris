#![allow(mixed_script_confusables)]

pub mod fleet;
pub mod game_map;
pub mod object_id;
pub mod states;
pub mod ui;

use bevy::prelude::*;
use states::{complete_setup, AppState, AppStateLoading};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .enable_state_scoped_entities::<AppState>()
            .add_computed_state::<AppStateLoading>()
            .add_systems(PostStartup, complete_setup);
    }
}
