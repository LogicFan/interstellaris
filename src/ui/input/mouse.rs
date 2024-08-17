use crate::ui::PrimaryCamera;
use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        ButtonState,
    },
    prelude::*,
    render::camera::RenderTarget,
    window::{CursorGrabMode, PrimaryWindow, WindowRef},
};
use bevy_mod_picking::{
    pointer::{InputMove, InputPress, Location},
    prelude::{Pickable, PointerButton, PointerId, PointerLocation},
    PointerBundle,
};

use super::parsed::ParsedMouseInput;

#[derive(Component, Copy, Clone, Debug)]
pub struct UiPointer;

#[derive(Component, Copy, Clone, Debug)]
pub struct MousePointer;

/// Spawn a [ImageBundle] that represents a pointer.
/// # Schedule
/// [Startup], after camera is initialized
pub(super) fn setup(
    mut commands: Commands,
    mut q_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
    assets: Res<AssetServer>,
) {
    let mut window = q_window
        .get_single_mut()
        .expect("Fail to find primary window.");

    let texture = assets.load("menu_ui/mouse/cursor.png");
    commands.spawn((
        ImageBundle {
            image: UiImage::new(texture),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                ..default()
            },
            z_index: ZIndex::Global(1024),
            ..default()
        },
        TargetCamera(q_camera.single()),
        UiPointer,
        Pickable::IGNORE,
    ));

    commands.spawn((PointerBundle::new(PointerId::Mouse), MousePointer));

    window.1.cursor.visible = false;
    window.1.cursor.grab_mode = CursorGrabMode::Confined;
}

pub(super) fn post_setup(
    q_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut ew_motion: EventWriter<InputMove>,
    mut parsed: ResMut<ParsedMouseInput>,
) {
    let window = q_window.get_single().expect("Fail to find primary window.");
    let viewport = Vec2::new(window.1.width(), window.1.height()) - 1.0;
    parsed.refresh_motion(viewport);

    // move the pointer to the center of the screen
    let center = 0.5 * Vec2::new(window.1.width(), window.1.height());
    let delta = parsed.set_motion(center);
    
    ew_motion.send(InputMove::new(
        PointerId::Mouse,
        Location {
            target: RenderTarget::Window(WindowRef::Primary)
                .normalize(Some(window.0))
                .unwrap(),
            position: parsed.position,
        },
        delta,
    ));

    parsed.update_motion();
}

/// Convert mouse motion events.
/// # Schedule
/// [First]
pub(super) fn motion(
    q_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut er_motion: EventReader<MouseMotion>,
    mut ew_motion: EventWriter<InputMove>,
    mut parsed: ResMut<ParsedMouseInput>,
) {
    let window = q_window
        .get_single()
        .expect("Fail to find primary window in Bevy");
    let viewport = Vec2::new(window.1.width(), window.1.height()) - 1.0;
    parsed.refresh_motion(viewport);

    for event in er_motion.read() {
        let delta = parsed.set_motion(event.delta);

        ew_motion.send(InputMove::new(
            PointerId::Mouse,
            Location {
                target: RenderTarget::Window(WindowRef::Primary)
                    .normalize(Some(window.0))
                    .unwrap(),
                position: parsed.position,
            },
            delta,
        ));
    }

    parsed.update_motion();
}

pub(super) fn wheel(mut er_wheel: EventReader<MouseWheel>, mut parsed: ResMut<ParsedMouseInput>) {
    for event in er_wheel.read() {}
}

/// Convert mouse button events.
/// # Schedule
/// [First]
pub(super) fn button(
    mut er_button: EventReader<MouseButtonInput>,
    mut ew_button: EventWriter<InputPress>,
) {
    for event in er_button.read() {
        let button = match event.button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            MouseButton::Other(_) => continue,
            MouseButton::Back => continue,
            MouseButton::Forward => continue,
        };

        match event.state {
            ButtonState::Pressed => {
                ew_button.send(InputPress::new_down(PointerId::Mouse, button));
            }
            ButtonState::Released => {
                ew_button.send(InputPress::new_up(PointerId::Mouse, button));
            }
        }
    }
}

/// Sync pointer position and its image
/// # Schedule
/// [Update], after [PointerMap] is updated.
pub(super) fn sync(
    mut q_target: Query<&mut Style, With<UiPointer>>,
    q_source: Query<&PointerLocation, With<MousePointer>>,
) {
    let source = q_source
        .get_single()
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
