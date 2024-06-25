use crate::utils::*;
use crate::planetary_system::api::Coordinate;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Fleet;

#[derive(Debug, Clone, Copy, Component)]
pub struct Owner(pub ObjectRef);

#[derive(Debug, Clone, Component)]
pub struct Vessels(pub Vec<ObjectRef>);

#[derive(Debug, Clone, Bundle)]
pub struct LFleet {
    pub marker: Fleet,
    pub id: ObjectId,
    pub transform: Transform,
    pub vessels: Vessels,
    pub owner: Owner,
    pub coordinate: Coordinate,
}

#[derive(Bundle, Clone)]
pub struct VFleet {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}


