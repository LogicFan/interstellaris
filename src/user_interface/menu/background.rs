use crate::user_interface::PrimaryCamera;
use bevy::prelude::*;

/// A marker component for background image.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MenuBackground;

/// Spawn the background for Menu.
/// # Schedule
/// `OnEnter(AppState::MenuScene)`
pub fn spawn_menu_background(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image = assets.load("background/background_0.png");
    let camera = q_camera.single();

    commands.spawn((
        MenuBackground,
        TargetCamera(camera),
        ImageBundle {
            image: UiImage {
                texture: image,
                ..default()
            },
            style: Style {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                min_width: Val::Vw(100.0),
                min_height: Val::Vh(100.0),
                max_width: Val::Vh(ASPECT_RATIO * 100.0),
                max_height: Val::Vw(ASPECT_RATIO.recip() * 100.0),
                aspect_ratio: Some(ASPECT_RATIO),
                ..default()
            },
            ..default()
        },
    ));
}

/// Despawn the background for Menu.
/// # Schedule
/// `OnExit(AppState::MenuScene)`
pub fn despawn_menu_background(
    mut commands: Commands,
    q_background: Query<Entity, With<MenuBackground>>,
) {
    for entity in q_background.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
