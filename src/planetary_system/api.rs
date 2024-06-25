use crate::utils::ObjectRef;
use bevy::prelude::Component;

/// Determine which coordinate system the entity is using. If it's None, then
/// a global coordinate system is used; otherwise, the coordinate system of
/// `ObjectRef` planetary system is used.
#[derive(Debug, Clone, Copy, Component)]
pub struct Coordinate(pub Option<ObjectRef>);
