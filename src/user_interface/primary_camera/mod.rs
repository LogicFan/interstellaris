//! This module controls the primary camera. There are two mode of
//! camera motion,
//!
//! # Free Motion Mode
//! In the free motion mode, all motion are described with respect to
//! invariant plane (i.e. z=0 plane).
//!
//! - if the cursor is at the edge of the screen, the camera will move
//!     parallel against z=0 plane. The constraint is based on the
//!     intersection point between camera local-z and z=0 plane.
//! - the mouse wheel scroll will move the camera along the local-z axis.
//!     The constraint is based on the distance between the camera
//!     and intersection point between camera local-z and z=0 plane.
//! - if the middle button is hold, then the cursor will stop moving, and
//!     x-axis movement will rotate against z-axis; y-axis movement will
//!     rotate the axis that is perpendicular to the plane of z-axis and
//!     local-z-axis. There is no constraint on x-axis movement. The
//!     constraint of y-axis movement is controlled by the angle between
//!     local-z-axis and z=0 plane.
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
use bevy::prelude::*;
use std::f32::consts::PI;

pub use super::settings::InputSettings;

pub mod free_motion;

/// The marker component of primary camera.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PrimaryCamera;

/// The marker component of the parent of the primary camera. Should always
/// at z=0 plane in free motion mode.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PrimaryCameraOrigin;

/// describe the motion mode of the camera
#[derive(Resource, Clone, Copy, Debug)]
pub enum PrimaryCameraMotionMode {
    MenuScene,
    FreeMotion {
        min_h: f32,
        max_h: f32,

        max_θ: f32,
        max_r: f32,
    },
    Transition {
        a: Vec3,
        b: Vec3,
        c: Vec3,
        d: Vec3,
        target: Vec3,
    },
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, SystemSet)]
pub struct PrimaryCameraSystemSet;

/// Spawn a new primary camera.
/// # Schedule
/// `Startup`
pub fn spawn_primary_camera(
    mut commands: Commands,
    mut r_motion_mode: ResMut<PrimaryCameraMotionMode>,
) {
    // TODO: the initial state should be menu scene, and switch to
    // free motion when we enter the game.
    *r_motion_mode = PrimaryCameraMotionMode::FreeMotion {
        min_h: 30.0,
        max_h: 100.0,
        max_θ: PI / 3.0,
        max_r: 30.0,
    };

    let camera = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            PrimaryCamera,
        ))
        .id();

    let _ = commands
        .spawn((TransformBundle::default(), PrimaryCameraOrigin))
        .add_child(camera);
}
