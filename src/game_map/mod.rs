pub mod galaxy;
pub mod gen;
pub mod planetary_system;

use crate::utils::object_id::ObjectRef;
use bevy::prelude::Component;

/// Determine which coordinate system the entity is using. If it's None, then
/// a global coordinate system is used; otherwise, the coordinate system of
/// `ObjectRef` planetary system is used.
#[derive(Component, Copy, Clone, Debug)]
pub struct Coordinate(pub Option<ObjectRef>);
