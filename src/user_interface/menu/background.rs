use crate::user_interface::PrimaryCamera;
use bevy::prelude::*;

/// A marker component for background image.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MenuBackground;

/// Spawn the background for Menu.
/// # Schedule
/// `Startup`, TODO: change to on_enter(AppState::MenuScene)
pub fn spawn_menu_background(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let image = assets.load("menu/background_0.png");
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
                position_type: PositionType::Absolute,
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn despawn_menu_background(
    mut commands: Commands,
    q_background: Query<Entity, With<MenuBackground>>,
) {
    let background = q_background.single();

    commands.entity(background).despawn();
}
