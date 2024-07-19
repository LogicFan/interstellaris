use crate::game_map::Coordinate;
use crate::utils::object_id::*;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub struct Fleet;

#[derive(Component, Copy, Clone, Debug)]
pub struct Owner(pub ObjectRef);

#[derive(Component, Clone, Debug)]
pub struct Vessels(pub Vec<ObjectRef>);

#[derive(Clone, Debug, Bundle)]
pub struct LFleet {
    pub marker: Fleet,
    pub id: ObjectId,
    pub transform: Transform,
    pub vessels: Vessels,
    pub owner: Owner,
    pub coordinate: Coordinate,
}

#[derive(Clone, Bundle)]
pub struct VFleet {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
