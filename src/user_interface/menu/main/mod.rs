use crate::user_interface::PrimaryCamera;
use bevy::prelude::*;

/// A marker component for background image.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MainMenuNode;

/// Spawn the background for Menu.
/// # Schedule
/// `OnEnter(AppState::MenuScene)`
pub fn spawn_main_menu(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let image = assets.load("menu/background_0.png");
    let camera = q_camera.single();

    commands.spawn((
        MainMenuNode,
        TargetCamera(camera),
        ImageBundle {
            image: UiImage {
                texture: image,
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::Center,
                width: Val::Percent(100.0),
                aspect_ratio: Some(3840.0 / 2160.0),
                ..default()
            },
            ..default()
        },
    ));
}

/// Despawn the background for Menu.
/// # Schedule
/// `OnExit(AppState::MenuScene)`
pub fn despawn_main_menu(
    mut commands: Commands,
    q_background: Query<Entity, With<MainMenuNode>>,
) {
    for entity in q_background.iter() {
        commands.entity(entity).despawn();
    }
}
