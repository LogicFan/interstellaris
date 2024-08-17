//! This module controls the primary camera. There are two mode of
//! camera motion,
//!
//! # Transition Mode
//! This mode only happens when we want to move from global map view to
//! local map view or vice versa.
//!
//! - for enter a local map, the start point is current position, the finish
//!     point is local map with position (0, 0, z) looking at (0, 0, 0), where
//!     z is computed based on the local map size.
//! - for leave a local map, the start point is current position, the finish
//!     point is (x0, y0, z) looking at (x0, y0, z0) where the z is based onm
//!     global map size and (x0, y0, z0) is the local map position in
//!     global map.
//!
//! The path should be a spline and the motion should be
//! accelerate-constant-decelerate. It should also be possible to terminate
//! during the motion and smoothly move back the initial position.
use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::prelude::*;
mod free_motion;
pub use crate::ui::settings::InputSettings;
pub use free_motion::Controller as PrimCamFreeMotion;

use super::CameraSet;

/// The marker component of primary camera.
#[derive(Component, Copy, Clone, Default, Debug)]
pub struct PrimaryCamera;

/// Spawn a new primary camera.
/// # Schedule
/// [PreStartup]
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 0.0, 50.0),
            ..default()
        },
        PrimaryCamera,
        BloomSettings::NATURAL,
    ));
}

pub struct PrimaryCameraPlugin;

impl Plugin for PrimaryCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup)
            .add_systems(Update, free_motion::slide.in_set(CameraSet::Motion));
    }
}
