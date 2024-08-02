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
use bevy::{core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}, prelude::*};
mod free_motion;

pub use crate::ui::settings::InputSettings;

pub use free_motion::*;

/// The marker component of primary camera.
#[derive(Component, Copy, Clone, Default, Debug)]
pub struct PrimaryCamera;

/// The marker component of the parent of the primary camera. Should always
/// at z=0 plane in free motion mode.
#[derive(Component, Copy, Clone, Default, Debug)]
pub struct CameraOrigin;

/// describe the motion mode of the camera
#[derive(Resource, Copy, Clone, Debug)]
pub enum MotionMode {
    NoMotion,
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

/// Spawn a new primary camera.
/// # Schedule
/// `Startup`
pub fn spawn_primary_camera(mut commands: Commands) {
    let camera = commands
        .spawn((
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
        ))
        .id();

    let _ = commands
        .spawn((TransformBundle::default(), CameraOrigin))
        .add_child(camera);
}
