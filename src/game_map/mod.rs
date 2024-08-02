pub mod galaxy;
pub mod gen;
pub mod planetary_system;

use crate::utils::ObjectRef;
use bevy::prelude::Component;

/// Determine which coordinate system the entity is using. If it's None, then
/// a global coordinate system is used; otherwise, the coordinate system of
/// `ObjectRef` planetary system is used.
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Coordinate {
    Galaxy(ObjectRef),
    PlnSys(ObjectRef),
}
