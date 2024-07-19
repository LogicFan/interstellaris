use super::{
    galaxy::gen::{handle_galaxy_gen_task, init_galaxy_gen_task},
    planetary_system::gen::spawn_planetary_systems,
};
use crate::{states::LoadSource, AppState};
use bevy::prelude::*;

#[derive(SubStates, Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[source(AppState = AppState::Loading(LoadSource::Generation))]
pub enum GenState {
    #[default]
    InitGalaxy,
    InitPlnSys,
}

/// The plugin for game map generation.
pub struct GampMapGenPlugin;

impl Plugin for GampMapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GenState>()
            .add_systems(OnEnter(GenState::InitGalaxy), init_galaxy_gen_task)
            .add_systems(
                Update,
                handle_galaxy_gen_task.run_if(in_state(GenState::InitGalaxy)),
            )
            .add_systems(OnEnter(GenState::InitPlnSys), spawn_planetary_systems);
    }
}
