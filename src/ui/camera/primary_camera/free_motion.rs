//! Control the camera when it is in free motion mode.
//!
//! # Motion Control
//! - if the cursor is at the edge of the screen, the camera will move
//!     parallel against z=z0 plane.
//! - the mouse wheel scroll will move the camera along the local-z axis.
//!     The constraint is based on the distance between the camera
//!     and intersection point between camera local-z and z=0 plane.
//! - if the middle button is hold, then the cursor will stop moving, camera
//!     will rotate accordingly.
//!
//! # Constraint
//! - height need to be between `-half_size.z` and `half_size.z + min(half_size.x, half_size.y) / tan θ`
//!     where θ is the camera FOV.
//! - for a given z, adjust x (and y) to the the minimal of `half_size.x` or
//! `half_size.x - (half_size.z - z) * tan θ`, where θ is the camera FOV.
//! - if camera is in the `half_size`, then the rotation is not restricted
//! - otherwise, the rotation is constrained by tan^(-1) ((half_size.x - x) / z).
use super::PrimaryCamera;
use crate::ui::input::ParsedMouseInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct Controller {
    pub half_size: Vec3,
}

/// Control the [PrimaryCamera] horizontal movement (a.k.a slide).
/// # Schedule
/// [PostUpdate], we want to move it after all ray-cast
/// computation are completed.
pub fn slide(
    mut q_camera: Query<&mut Transform, (With<PrimaryCamera>, With<Controller>)>,
    input: Res<ParsedMouseInput>,
) {
    if let Some(mut transform) = q_camera.get_single_mut().ok() {
        let delta = transform.local_x().xy().normalize() * input.on_border.x
            - transform.local_y().xy().normalize() * input.on_border.y;

        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}

// /// Control the [PrimaryCamera] horizontal movement (a.k.a slide).
// /// # Schedule
// /// [PostUpdate], we want to move it after all ray-cast
// /// computation are completed.
// pub fn slide(
//     mut q_camera: Query<&mut Transform, (With<PrimaryCamera>, With<Controller>)>,
//     input: Res<ParsedMouseInput>,
// ) {
//     if let Some(mut transform) = q_camera.get_single_mut().ok() {
//         let delta = transform.local_x().xy().normalize() * input.on_border.x
//             - transform.local_y().xy().normalize() * input.on_border.y;

//         transform.translation.x += delta.x;
//         transform.translation.y += delta.y;
//     }
// }

// /// Control the [PrimaryCamera] horizontal movement (a.k.a slide).
// /// # Schedule
// /// [PostUpdate], we want to move it after all ray-cast
// /// computation are completed.
// pub fn constraint(
//     mut q_camera: Query<(&mut Transform, &Controller), With<PrimaryCamera>>,
//     input: Res<ParsedMouseInput>,
// ) {
//     if let Some((mut transform, ctrl)) = q_camera.get_single_mut().ok() {

//         let delta = transform.local_x().xy().normalize() * input.on_border.x
//             - transform.local_y().xy().normalize() * input.on_border.y;

//         transform.translation.x += delta.x;
//         transform.translation.y += delta.y;
//     }
// }

// /// Control primary camera zoom.
// /// # Schedule
// /// `PostUpdate`, if `is_free_motion` is true.
// pub fn zoom_main_camera(
//     mut q_camera: Query<&mut Transform, With<PrimaryCamera>>,
//     mut e_mouse_wheel: EventReader<MouseWheel>,
//     r_motion_mode: Res<MotionMode>,
//     r_settings: Res<InputSettings>,
// ) {
//     let (min_h, max_h) = match *r_motion_mode {
//         MotionMode::FreeMotion { min_h, max_h, .. } => (min_h, max_h),
//         other => panic!("Unexpected value {:?}", other),
//     };

//     let mut transform = q_camera.single_mut();

//     use bevy::input::mouse::MouseScrollUnit;
//     for e in e_mouse_wheel.read() {
//         let mut dz = match e.unit {
//             MouseScrollUnit::Line => -e.y,
//             MouseScrollUnit::Pixel => -e.y,
//         };

//         // apply sensitivity settings
//         dz *= r_settings.mouse_scroll_sensitivity;

//         // apply constraint
//         let mut z = transform.translation.z + dz;
//         z = z.clamp(min_h, max_h);

//         transform.translation.z = z;
//     }
// }

// /// Control primary camera rotation.
// /// # Schedule
// /// `PostUpdate`, if `is_free_motion` is true.
// pub fn rotate_main_camera(
//     mut q_camera_origin: Query<&mut Transform, With<CameraOrigin>>,
//     mut q_window: Query<&mut Window, With<PrimaryWindow>>,
//     mut e_mouse_motion: EventReader<MouseMotion>,
//     mut l_cursor_position: Local<Option<Vec2>>,
//     r_mouse_button: Res<ButtonInput<MouseButton>>,
//     r_motion_mode: Res<MotionMode>,
//     r_settings: Res<InputSettings>,
// ) {
//     let max_θ = match *r_motion_mode {
//         MotionMode::FreeMotion { max_θ, .. } => max_θ,
//         other => panic!("Unexpected value {:?}", other),
//     };

//     let mut window = q_window.single_mut();

//     if r_mouse_button.pressed(MouseButton::Middle) {
//         // lock the cursor
//         window.set_cursor_position(*l_cursor_position);

//         // find mouse movement
//         let mut delta = Vec2::ZERO;
//         for e in e_mouse_motion.read() {
//             delta += e.delta;
//         }

//         // apply sensitivity settings
//         delta *= 0.01 * r_settings.mouse_motion_sensitivity;

//         // rotate horizontally
//         let mut transform = q_camera_origin.single_mut();
//         transform.rotate_z(delta.x);

//         // rotate vertically, apply constraint
//         // TODO: optimize computation here, we can skip first and third
//         // angle computation
//         let (_, θ, _) = transform.rotation.to_euler(EulerRot::ZXY);
//         delta.y = delta.y.clamp(-θ, max_θ - θ);

//         transform.rotate_local_x(delta.y);
//     } else {
//         *l_cursor_position = window.cursor_position();
//     }
// }
