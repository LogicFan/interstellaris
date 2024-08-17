pub mod gen;

use bevy::prelude::*;

/// A marker component for galaxy in game map.
#[derive(Component, Copy, Clone, Debug)]
pub struct Galaxy;

/// A marker component to indicate the primary galaxy.
#[derive(Component, Copy, Clone, Debug)]
pub struct PrimaryGalaxy;
