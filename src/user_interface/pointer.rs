//! This module contains an implementation of virtual cursor pointer. This is
//! because macOS doesn’t support CursorGrabMode::Confined

use super::MainCamera;
use bevy::input::mouse::MouseMotion;
use bevy::math::Vec2;
use bevy::prelude::default;
use bevy::prelude::AssetServer;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::EventReader;
use bevy::prelude::ImageBundle;
use bevy::prelude::PositionType;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Style;
use bevy::prelude::TargetCamera;
use bevy::prelude::UiImage;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::render::view::Visibility;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;
use bevy::window::Window;

/// Singleton. A marker component of virtual cursor pointer.
/// - `x`: the `left` value of cursor ui node, in terms of pixel
/// - `y`: the `top` value of cursor ui node, in terms of pixel
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryPointer {
    pub x: f32,
    pub y: f32,
}

/// Spawn a new virtual cursor pointer
/// # Schedule
/// `Startup`, after MainCamera is initialized
pub fn spawn_virtual_pointer(
    mut commands: Commands,
    assets: Res<AssetServer>,
    camera: Query<Entity, With<MainCamera>>,
) {
    let image_handle = assets.load("ui/pointer.png");
    let camera_id = camera.get_single().unwrap();

    commands.spawn((
        PrimaryPointer::default(),
        TargetCamera(camera_id),
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(16.0),
                width: Val::Px(16.0),
                ..default()
            },
            image: UiImage::new(image_handle),
            visibility: Visibility::Hidden,
            ..default()
        },
    ));
}

/// enable the virtual cursor pointer
/// # Schedule
/// after VirtualPointer is spawned
pub fn enable_virtual_pointer(
    mut q_pointer: Query<(&mut PrimaryPointer, &mut Visibility)>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let (mut pointer, mut visibility) = q_pointer.single_mut();
    let mut window = q_window.single_mut();
    *visibility = Visibility::Visible;

    match window.cursor_position() {
        Some(p) => {
            pointer.x = p.x;
            pointer.y = p.y;
        }
        None => {
            pointer.x = window.width() / 2.0;
            pointer.y = window.height() / 2.0;
        }
    }

    window.cursor.grab_mode = CursorGrabMode::Confined;
    window.cursor.visible = false;
}

/// disable the virtual cursor pointer
/// # Schedule
/// after VirtualPointer is spawned
pub fn disable_virtual_pointer(
    mut q_pointer: Query<(&PrimaryPointer, &mut Visibility)>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let (pointer, mut visibility) = q_pointer.single_mut();
    let mut window = q_window.single_mut();
    *visibility = Visibility::Hidden;

    window.cursor.grab_mode = CursorGrabMode::None;
    window.set_cursor_position(Some(Vec2::new(pointer.x, pointer.y)));
    window.cursor.visible = false;
}

/// disable the virtual cursor pointer
/// # Schedule
/// after update_virtual_pointer
pub fn sync_virtual_pointer(
    mut q_style: Query<&mut Style, With<PrimaryPointer>>,
    q_pointer: Query<&PrimaryPointer>,
) {
    let mut style = q_style.single_mut();
    let pointer = q_pointer.single();

    style.top = Val::Px(pointer.y);
    style.left = Val::Px(pointer.x);
}

/// Spawn a new virtual cursor pointer
/// # Schedule
/// `Startup`, after MainCamera is initialized
pub fn update_virtual_pointer(
    mut q_pointer: Query<&mut PrimaryPointer>,
    mut e_motion: EventReader<MouseMotion>,
) {
    let mut pointer = q_pointer.single_mut();
    // TODO: use mouse_motion_sensitivity
    for e in e_motion.read() {
        pointer.x += e.delta.x;
        pointer.y += e.delta.y;
    }
}
