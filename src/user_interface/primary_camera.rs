//! This module controls the primary camera. There are two mode of 
//! camera motion, 
//! 
//! # Free Motion Mode
//! In the free motion mode, all motion are described with respect to 
//! invariant plane (i.e. xy-plane). 
//! 
//! - if the cursor is at the edge of the screen, the camera will move
//!     parallel against xy-plane. The constraint is based on the 
//!     intersection point between camera local-z and xy-plane.
//! - the mouse wheel scroll will move the camera along the local-z axis.
//!     The constraint is based on the distance between the camera
//!     and intersection point between camera local-z and xy-plane.
//! - if the middle button is hold, then the cursor will stop moving, and
//!     x-axis movement will rotate against z-axis; y-axis movement will 
//!     rotate the axis that is perpendicular to the plane of z-axis and
//!     local-z-axis. There is no constraint on x-axis movement. The 
//!     constraint of y-axis movement is controlled by the angle between 
//!     local-z-axis and xy-plane.
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

use super::settings::InputSettings;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, SystemSet)]
pub struct PrimaryCameraSystemSet;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryCamera;

#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct PrimaryCameraConstraint {
    pub min_height: f32,
    pub max_height: f32,

    /// the minimal angle from Z = 0 plane.
    pub min_angle: f32,

    pub min_xy: Vec2,
    pub max_xy: Vec2,
}

impl PrimaryCameraConstraint {
    pub fn new(h0: f32, h1: f32, r: f32) -> Self {
        Self {
            min_height: h0,
            max_height: h1,
            min_angle: PI / 6.0,
            min_xy: Vec2::NEG_ONE * r,
            max_xy: Vec2::ONE * r,
        }
    }

    pub fn set_r(&mut self, r: f32) -> &mut Self {
        self.min_xy = Vec2::NEG_ONE * r;
        self.max_xy = Vec2::ONE * r;
        self
    }
}

/// Spawn a new primary camera.
/// # Schedule
/// `Startup`
pub fn spawn_main_camera(
    mut commands: Commands,
    mut r_constraint: ResMut<PrimaryCameraConstraint>,
) {
    *r_constraint = PrimaryCameraConstraint::new(20.0, 100.0, 30.0);

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        PrimaryCamera,
    ));
}

/// Control primary camera horizontal movement.
/// # Schedule
/// `PostUpdate`
pub fn move_main_camera(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    r_settings: Res<InputSettings>,
) {
    let mut transform = q_camera.single_mut();
    let window = q_window.single();

    match window.cursor_position() {
        None => (),
        Some(p) => {
            let dx = f32::from(p.x >= window.width() - 1.0) - f32::from(p.x <= 0.0);
            let dy = f32::from(p.y >= window.height() - 1.0) - f32::from(p.y <= 0.0);
            let mut delta = Vec2::ZERO;
            // remove z-component
            delta += transform.local_x().xy().normalize() * dx;
            delta -= transform.local_y().xy().normalize() * dy;
            // apply sensitivity settings
            delta *= r_settings.mouse_motion_sensitivity;

            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }
}

/// Control primary camera zoom.
/// # Schedule
/// `PostUpdate`
pub fn zoom_main_camera(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    mut e_mouse_wheel: EventReader<MouseWheel>,
    r_settings: Res<InputSettings>,
) {
    let mut transform = q_camera.single_mut();

    // TODO: limit movement in a box.
    use bevy::input::mouse::MouseScrollUnit;
    for e in e_mouse_wheel.read() {
        let dz = match e.unit {
            MouseScrollUnit::Line => -e.y,
            MouseScrollUnit::Pixel => -e.y,
        };
        let mut delta = Vec3::ZERO;
        delta += transform.local_z() * dz;
        // apply sensitivity settings
        delta *= r_settings.mouse_scroll_sensitivity;

        transform.translation += delta;
    }
}

/// Control primary camera rotation.
/// # Schedule
/// `PostUpdate`
pub fn rotate_main_camera(
    mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut e_mouse_motion: EventReader<MouseMotion>,
    mut l_cursor: Local<Option<Vec2>>,
    r_settings: Res<InputSettings>,
    r_mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let mut transform = q_camera.single_mut();
    let mut window = q_window.single_mut();

    if r_mouse_button.pressed(MouseButton::Middle) {
        // lock the cursor
        window.set_cursor_position(*l_cursor);

        // find mouse movement
        let mut delta = Vec2::ZERO;
        for e in e_mouse_motion.read() {
            delta += e.delta;
        }
        // y is inverted
        delta.y = -delta.y;

        // TODO: fix rotation
        // dx rotate global_z
        // dx rotate on plane of local_z and global_z

        // do not rotate if there is no mouse movement
        // to avoid NaN error
        if delta != Vec2::ZERO {
            let local_x = transform.local_x();
            let local_y = transform.local_y();
            let local_z = transform.local_z();

            // compute rotation origin
            let offset = local_z * (transform.translation.z / local_z.z);
            let origin = transform.translation - offset;
            println!("rotation origin is: {:?}", origin);

            // compute the rotation quaternion
            let rotation = local_x * delta.y - local_y * delta.x;
            let angle = rotation.length() * 0.01 * r_settings.mouse_motion_sensitivity;
            let axis = rotation.normalize();
            println!("rotation angle is: {:?}", angle);
            println!("rotation axis is: {:?}", axis);

            transform.rotate_around(origin, Quat::from_axis_angle(axis, angle));
        }
    } else {
        *l_cursor = window.cursor_position();
    }
}
