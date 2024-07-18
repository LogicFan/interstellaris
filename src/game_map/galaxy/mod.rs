pub mod gen;

use bevy::prelude::*;

/// A marker component for galaxy in game map
#[derive(Component, Debug, Clone, Copy)]
pub struct Galaxy;

/// A marker component for galaxy in game map
#[derive(Component, Debug, Clone, Copy)]
pub struct GalaxySize {
    pub min: Vec3,
    pub max: Vec3,
}
