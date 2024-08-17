use super::galaxy::gen::{handle_galaxy_gen_task, init_galaxy_gen_task};
use super::galaxy::Galaxy;
use super::planetary_system::gen::spawn_planetary_systems;
use super::{BoundingSize, Coordinate};
use crate::states::AppStateLoading;
use crate::ui::camera::PrimCamFreeMotion;
use crate::utils::{ObjectId, ObjectRef};
use crate::{states::LoadSource, ui::PrimaryCamera, AppState};
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

pub fn setup_primary_camera(
    q_galaxy: Query<(Entity, &ObjectId, &BoundingSize), With<Galaxy>>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
    mut commands: Commands,
) {
    let camera = q_camera.single();

    for (entity, object_id, size) in q_galaxy.iter() {
        commands
            .entity(camera)
            .insert(Coordinate::Galaxy(ObjectRef::new(entity, *object_id)))
            .insert(PrimCamFreeMotion { half_size: size.half_size });
        break;
    }
}

impl Plugin for GampMapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GenState>()
            .add_systems(OnEnter(GenState::InitGalaxy), init_galaxy_gen_task)
            .add_systems(
                Update,
                handle_galaxy_gen_task.run_if(in_state(GenState::InitGalaxy)),
            )
            .add_systems(OnEnter(GenState::InitPlnSys), spawn_planetary_systems)
            .add_systems(OnExit(AppStateLoading), setup_primary_camera);
    }
}
