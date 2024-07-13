use crate::ui::PrimaryCamera;
use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;
use sickle_ui::prelude::{generated::*, UiBuilderExt, UiRoot};

/// A marker component for background image.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MenuBackground(i64);

/// Spawn the background for the menu.
/// # Schedule
/// `OnEnter(AppState::MenuScene)`
pub fn spawn_background(
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
                color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                texture: image,
                ..default()
            },
            ..default()
        })
        .insert(TargetCamera(camera))
        .insert(Name::new("Menu Background"))
        .insert(MenuBackground(0))
        .insert(Pickable::IGNORE)
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

pub fn update_background(
    assets: Res<AssetServer>,
    mut q_background: Query<(&mut UiImage, &mut MenuBackground)>
) {
    info!("update background!");
    let (mut ui_image, mut background) = q_background.single_mut();
    background.0 = (background.0 + 1) % 2;
    let image = assets.load(format!("background/background_{}.png", background.0));
    ui_image.texture = image;
}