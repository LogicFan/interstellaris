use super::settings::InputSettings;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, SystemSet)]
pub struct PrimaryCameraSystemSet;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryCamera;

#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct CameraMotionConstraint {
    pub min_height: f32,
    pub max_height: f32,

    /// the minimal angle from Z = 0 plane.
    pub min_angle: f32,

    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

/// Spawn a new primary camera.
/// # Schedule
/// `Startup`
pub fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0)
                .with_rotation(Quat::from_rotation_x(0.1)),
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
    const XY: Vec3 = Vec3::new(1.0, 1.0, 0.0);

    let mut transform = q_camera.single_mut();
    let window = q_window.single();

    // TODO: limit movement in a box.
    // TODO: MAYBE change speed based on height.
    match window.cursor_position() {
        None => (),
        Some(p) => {
            let dx = f32::from(p.x >= window.width() - 1.0) - f32::from(p.x <= 0.0);
            let dy = f32::from(p.y <= 0.0) - f32::from(p.y >= window.height() - 1.0);
            let mut delta = Vec3::ZERO;
            // remove z-component
            delta += (Vec3::from(transform.local_x()) * XY).normalize() * dx;
            delta += (Vec3::from(transform.local_y()) * XY).normalize() * dy;
            // apply sensitivity settings.
            delta *= r_settings.mouse_motion_sensitivity;

            transform.translation += delta;
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
        // y is actually inverted
        delta.y = -delta.y;

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
