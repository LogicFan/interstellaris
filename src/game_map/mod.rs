mod gen;

pub use gen::init_galaxy;

use crate::utils::ObjectRef;
use bevy::prelude::Component;

/// Determine which coordinate system the entity is using. If it's None, then
/// a global coordinate system is used; otherwise, the coordinate system of
/// `ObjectRef` planetary system is used.
#[derive(Component, Debug, Clone, Copy)]
pub struct Coordinate(pub Option<ObjectRef>);
