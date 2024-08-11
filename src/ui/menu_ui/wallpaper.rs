//! The wallpapers for [crate::AppState::InMenu] and
//! [crate::AppState::Loading] states.

use crate::ui::PrimaryCamera;
use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;

/// A component for the wallpapers during [crate::AppState::InMenu]
/// and [crate::AppState::Loading].
///
/// Indicate an entity is wallpapers. Store all background
/// images to avoid loading time.
#[derive(Component, Clone, Default, Debug)]
pub(super) struct Wallpapers {
    idx: usize,
    images: Vec<Handle<Image>>,
}

impl Wallpapers {
    /// Create a new [Wallpapers] from a collection of images.
    fn new(images: Vec<Handle<Image>>) -> Self {
        Self {
            idx: images.len() - 1,
            images,
        }
    }

    /// return the next image and increment one.
    fn next(&mut self) -> Handle<Image> {
        self.idx = (self.idx + 1) % self.images.len();
        self.images[self.idx].clone()
    }
}

/// Spawn the background for the menu.
///
/// # Schedule
/// Enter [crate::AppState::InMenu].
pub(super) fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let camera = q_camera.single();
    let images = (0..2)
        .into_iter()
        .map(|i| assets.load(format!("menu_ui/wallpaper/{}.png", i)))
        .collect();
    let mut background = Wallpapers::new(images);

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    commands
        .spawn(ImageBundle {
            image: UiImage {
                texture: background.next(),
                ..default()
            },
            style: Style {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                min_height: Val::Vh(100.0),
                max_height: Val::Vw(ASPECT_RATIO.recip() * 100.0),
                min_width: Val::Vw(100.0),
                max_width: Val::Vh(ASPECT_RATIO * 100.0),
                aspect_ratio: Some(ASPECT_RATIO),
                ..default()
            },
            z_index: ZIndex::Global(-64),
            ..default()
        })
        .insert(background)
        .insert(Name::new("Menu Wallpaper"))
        .insert(TargetCamera(camera))
        .insert(Pickable::IGNORE);
}

/// Update background image to next image.
///
/// # Schedule
/// In [crate::AppState::InMenu], need to have [Wallpapers] resource.
pub(super) fn update(mut q_background: Query<(&mut UiImage, &mut Wallpapers), With<Wallpapers>>) {
    for (mut image, mut background) in q_background.iter_mut() {
        image.texture = background.next();
    }
}

/// Remove all background related components and resource.
///
/// # Schedule
/// Exit [crate::AppState::Loading].
pub(super) fn cleanup(mut commands: Commands, q_background: Query<Entity, With<Wallpapers>>) {
    for entity in q_background.iter() {
        commands.entity(entity).despawn();
    }
}
