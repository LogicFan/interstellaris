pub mod galaxy;
pub mod gen;
pub mod planetary_system;

use std::ops::Deref;

use crate::utils::ObjectRef;
use bevy::{math::Vec3, prelude::Component};

/// Determine which coordinate system the entity is using. If it's None, then
/// a global coordinate system is used; otherwise, the coordinate system of
/// `ObjectRef` planetary system is used.
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Coordinate {
    Galaxy(ObjectRef),
    PlnSys(ObjectRef),
}

impl Deref for Coordinate {
    type Target = ObjectRef;

    fn deref(&self) -> &Self::Target {
        match self {
            Coordinate::Galaxy(x) => x,
            Coordinate::PlnSys(x) => x,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct BoundingSize(pub Vec3);
