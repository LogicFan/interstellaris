use crate::ui::{menu_ui::AppState, PrimaryCamera};
use bevy::prelude::*;
use sickle_ui::prelude::*;

/// A marker component for background image.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MenuBackground;

/// Spawn the background for the menu.
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

    commands
        .ui_builder(UiRoot)
        .spawn(ImageBundle {
            image: UiImage {
                texture: image,
                ..default()
            },
            ..default()
        })
        .insert(TargetCamera(camera))
        .insert(Name::new("Menu Background"))
        .insert(MenuBackground)
        .style()
        .align_self(AlignSelf::Center)
        .justify_self(JustifySelf::Center)
        .min_height(Val::Vh(100.0))
        .max_height(Val::Vw(ASPECT_RATIO.recip() * 100.0))
        .min_width(Val::Vw(100.0))
        .max_width(Val::Vh(ASPECT_RATIO * 100.0))
        .aspect_ratio(Some(ASPECT_RATIO))
        .z_index(ZIndex::Global(-64));
}
