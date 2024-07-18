pub mod gen;

use crate::object_id::{ObjectId, ObjectRef};
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PlanetarySystem;

#[derive(Debug, Default, Clone, Component)]
pub struct Planets(pub Vec<ObjectRef>);

#[derive(Debug, Default, Clone, Bundle)]
pub struct PlanetarySystemBundle {
    pub marker: PlanetarySystem,
    pub id: ObjectId,
    pub transform: Transform,
    pub planets: Planets,
}

#[derive(Debug, Default, Bundle, Clone)]
pub struct VPlanetarySystemBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
