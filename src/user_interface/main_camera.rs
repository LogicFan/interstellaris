use super::settings::InputSettings;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, SystemSet)]
pub struct PrimaryCameraSystemSet;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryCamera;

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
