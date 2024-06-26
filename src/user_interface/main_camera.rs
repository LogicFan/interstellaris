use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct MainCamera;

/// Spawn a new main camera.
/// # Schedule
/// `PreStartup`
pub fn init_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0).with_rotation(Quat::from_rotation_x(0.1)),
            ..default()
        },
        MainCamera,
    ));
}

/// Control main camera movement. 
/// # Schedule
/// `Update`
pub fn update_main_camera(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut e_mouse_wheel: EventReader<MouseWheel>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut transform = q_camera.single_mut();
    let window = q_window.single();

    // TODO: use sensitivity settings.
    // TODO: limit movement in a box.

    // horizontal move
    match window.cursor_position() {
        None => (),
        Some(p) => {
            let dx = f32::from(p.x >= window.width() - 1.0) - f32::from(p.x <= 0.0);
            let dy = f32::from(p.y <= 0.0) - f32::from(p.y >= window.height() - 1.0);
            let mut delta = transform.local_x() * dx + transform.local_y() * dy;
            delta.z = 0.0;

            transform.translation += delta.normalize_or_zero();
        }
    }

    // zoom
    use bevy::input::mouse::MouseScrollUnit;
    for e in e_mouse_wheel.read() {
        let dz = match e.unit {
            MouseScrollUnit::Line => -e.y,
            MouseScrollUnit::Pixel => -e.y,
        };
        let delta = transform.local_z() * dz;
        
        transform.translation += delta.normalize_or_zero();
    }

    // rotate

}
