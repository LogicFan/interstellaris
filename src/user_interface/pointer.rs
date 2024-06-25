//! because macOS doesn’t support CursorGrabMode::Confined
//! we need to add our own virtual cursor pointer
use bevy::prelude::*;
use crate::user_interface::main_camera::MainCamera;

#[derive(Debug, Default, Clone, Copy, Component)]
struct VirtualPointer;

/// Spawn a new virtual cursor pointer
/// # Schedule
/// `Startup`
pub fn init_virtual_pointer(mut commands: Commands, assets: Res<AssetServer>, main_camera: Query<Entity, With<MainCamera>>) {
    let image_handle = assets.load("ui/pointer.png");
    let camera_id = main_camera.get_single().unwrap();

    commands.spawn((
        VirtualPointer,
        TargetCamera(camera_id),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(16.0),
                width: Val::Px(16.0),
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                ..default()
            },
            image: UiImage::new(image_handle),
            ..default()
        },
    ));
}
