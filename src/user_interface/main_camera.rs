use bevy::prelude::*;


#[derive(Debug, Default, Clone, Copy, Component)]
pub struct MainCamera;

/// Spawn a new main camera. 
/// # Schedule
/// `PreStartup`
pub fn init_main_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        MainCamera,
    ));
}

/// Control main camera movement. If a mouse stay 
/// at window edge, then move the camera horizontally.
/// # Schedule
/// `Update`
pub fn move_main_camera() {

}

/// Control main camera movement. If a mouse p oscroll ur down 
/// then zoom in or out the camera.
/// # Schedule
/// `Update`
pub fn zoom_main_camera() {

}

pub fn rotate_main_camera() {

}