//! In the free motion mode, all motion are described with respect to
//! z=z0 plane.
//!
//! - if the cursor is at the edge of the screen, the camera will move
//!     parallel against z=z0 plane.
//! - the mouse wheel scroll will move the camera along the local-z axis.
//!     The constraint is based on the distance between the camera
//!     and intersection point between camera local-z and z=0 plane.
//! - if the middle button is hold, then the cursor will stop moving, and
//!     x-axis movement will rotate against z-axis; y-axis movement will
//!     rotate the axis that is perpendicular to the plane of z-axis and
//!     local-z-axis. There is no constraint on x-axis movement. The
//!     constraint of y-axis movement is controlled by the angle between
//!     local-z-axis and z=0 plane.
use super::InputSettings;
use super::PrimaryCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct FreeMotionCtrl {
    half_size: Vec2,
}

impl FreeMotionCtrl {
    pub fn new(half_size: Vec2) -> Self {
        Self { half_size }
    }

    // TODO: add constraint
    fn horizontal_move(&self, transform: &mut Transform, delta: Vec2) {
        let mut xy = transform.translation.xy() + delta;
        xy = xy.clamp(-1.0 * self.half_size, self.half_size);

        transform.translation.x = xy.x;
        transform.translation.y = xy.y;
    }
}

/// Control primary camera horizontal movement.
/// # Schedule
/// `PostUpdate`, if `is_free_motion` is true, before transform propagation
pub fn run(
    mut q_camera: Query<(&mut Transform, &FreeMotionCtrl), With<PrimaryCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    r_settings: Res<InputSettings>,
) {
    let window = q_window.single();

    if let Some((mut transform, controller)) = q_camera.get_single_mut().ok() {
        match window.cursor_position() {
            None => (),
            Some(p) => {
                let dx = f32::from(p.x >= window.width() - 1.0) - f32::from(p.x <= 0.0);
                let dy = f32::from(p.y >= window.height() - 1.0) - f32::from(p.y <= 0.0);

                // remove z-component
                let mut delta = Vec2::ZERO;
                delta += transform.local_x().xy().normalize() * dx;
                delta -= transform.local_y().xy().normalize() * dy;
                delta *= r_settings.mouse_motion_sensitivity;

                controller.horizontal_move(&mut transform, delta);
            }
        }
    }
}

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
