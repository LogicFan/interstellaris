pub mod gen;

use bevy::prelude::*;

/// A marker component for galaxy in game map
#[derive(Component, Copy, Clone, Debug)]
pub struct Galaxy;

/// A marker component for galaxy in game map
#[derive(Component, Copy, Clone, Debug)]
pub struct GalaxySize {
    pub min: Vec3,
    pub max: Vec3,
}
