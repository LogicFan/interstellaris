pub mod gen;

use crate::utils::{ObjectId, ObjectRef};
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct PlanetarySystem;

#[derive(Component, Clone, Default, Debug)]
pub struct Planets(pub Vec<ObjectRef>);

#[derive(Clone, Default, Debug, Bundle)]
pub struct PlanetarySystemBundle {
    pub marker: PlanetarySystem,
    pub id: ObjectId,
    pub transform: Transform,
    pub planets: Planets,
}

#[derive(Clone, Default, Debug, Bundle)]
pub struct VPlanetarySystemBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
