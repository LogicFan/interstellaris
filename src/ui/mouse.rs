//! This module controls the behavior of the cursor.

use super::PrimaryCamera;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;
use bevy::window::Window;
use bevy_mod_picking::prelude::*;
use sickle_ui::prelude::*;

/// The plugin for mouse pointer.
pub struct MousePointerPlugin;

impl Plugin for MousePointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_pointer)
            .add_systems(Startup, setup_window)
            .add_systems(Update, sync_position);
    }
}

#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct UiPointer;

/// setup the virtual pointer
/// # Schedule
/// [Startup], after camera is initialized
fn setup_ui_pointer(
    mut commands: Commands,
    q_camera: Query<Entity, With<PrimaryCamera>>,
    assets: Res<AssetServer>,
) {
    let image = assets.load("menu_ui/mouse/cursor.png");

    commands
        .ui_builder(UiRoot)
        .spawn(ImageBundle {
            image: UiImage::new(image),
            ..default()
        })
        .insert(TargetCamera(q_camera.single()))
        .insert(UiPointer)
        .insert(Pickable::IGNORE)
        .style()
        .position_type(PositionType::Absolute)
        .top(Val::Px(0.0))
        .left(Val::Px(0.0))
        .width(Val::Px(20.0))
        .height(Val::Px(20.0))
        .z_index(ZIndex::Global(1000));
}

/// Hide window cursor since we are using
fn setup_window(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_window
        .get_single_mut()
        .expect("Find to find primary window.");
    window.cursor.visible = false;
}

/// Sync pointer position and its image
/// # Schedule
/// [Update], after [PointerMap] is updated.
fn sync_position(
    mut q_target: Query<&mut Style, With<UiPointer>>,
    q_source: Query<&PointerLocation>,
    pointer_map: Res<PointerMap>,
) {
    let pointer = pointer_map
        .get_entity(PointerId::Mouse)
        .expect("Fail to find mouse in PointerMap");
    let source = q_source
        .get(pointer)
        .expect("Fail to find mouse entity in Bevy");
    let mut target = q_target
        .get_single_mut()
        .expect("More than one UI pointer detected.");
    if let Some(location) = source.location() {
        target.top = Val::Px(location.position.y);
        target.left = Val::Px(location.position.x);
    } else {
        debug!("Pointer is outside of screen");
    }
}

/// confine pointer into window
/// # Schedule
/// [Update]
pub fn confine_pointer(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_window
        .get_single_mut()
        .expect("Find to find primary window.");
    window.cursor.grab_mode = CursorGrabMode::Confined;
}

/// release the confined pointer
/// # Schedule
/// [Update]
pub fn release_pointer(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_window
        .get_single_mut()
        .expect("Find to find primary window.");
    window.cursor.grab_mode = CursorGrabMode::None;
}

// pub fn enable_virtual_cursor(
//     mut commands: Commands,
//     mut q_window: Query<&mut Window, With<PrimaryWindow>>,
//     pointer_map: Res<PointerMap>,
// ) {
//     let mut window = q_window.single_mut();
//     window.cursor.grab_mode = CursorGrabMode::Confined;

//     if let Some(pointer) = pointer_map.get_entity(PointerId::Mouse) {
//         commands.entity(pointer).insert(bundle)
//     }
// }

// pub fn disable_virtual_cursor(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {}

// pub fn release_cursor(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
//     let mut window = q_window.single_mut();
//     window.cursor.grab_mode = CursorGrabMode::None;
// }
