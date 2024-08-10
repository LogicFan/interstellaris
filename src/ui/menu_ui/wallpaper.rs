//! The background image for [crate::AppState::InMenu] and
//! [crate::AppState::Loading]

use crate::ui::PrimaryCamera;
use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;
use sickle_ui::prelude::{generated::*, UiBuilder, UiBuilderExt, UiRoot};

/// A component for the background during [crate::AppState::InMenu]
/// and [crate::AppState::Loading].
///
/// Indicate an entity is menu background. Store all background
/// images to avoid loading time.
#[derive(Component, Clone, Default, Debug)]
pub struct BackgroundImage {
    current: usize,
    images: Vec<Handle<Image>>,
}

impl BackgroundImage {
    /// Create a new [BackgroundImages] from a collection of images.
    fn new(images: Vec<Handle<Image>>) -> Self {
        Self { current: 0, images }
    }

    /// Get the [Handle] of current background image.
    fn image(&self) -> Handle<Image> {
        self.images[self.current].clone()
    }

    /// Change to next image.
    fn next(&mut self) {
        self.current = (self.current + 1) % self.images.len();
    }
}

/// Spawn the background for the menu.
///
/// # Schedule
/// Enter [crate::AppState::InMenu].
pub fn spawn_background(
    mut commands: Commands,
    asserts: Res<AssetServer>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let camera = q_camera.single();

    let images = (0..2)
        .into_iter()
        .map(|i| asserts.load(format!("menu_ui/wallpaper/{}.png", i)))
        .collect();
    let background = BackgroundImage::new(images);

    commands
        .ui_builder(UiRoot)
        .wallpaper(UiImage {
            texture: background.image(),
            ..default()
        })
        .insert(TargetCamera(camera))
        .insert(Name::new("Menu Background"))
        .insert(background)
        .insert(Pickable::IGNORE)
        .style()
        .z_index(ZIndex::Global(-64));
}

/// Update background image to next image.
///
/// # Schedule
/// In [crate::AppState::InMenu], need to have [BackgroundImages] resource.
pub fn update_background(
    mut q_background: Query<(&mut UiImage, &mut BackgroundImage), With<BackgroundImage>>,
) {
    for (mut image, mut background) in q_background.iter_mut() {
        background.next();
        image.texture = background.image();
    }
}

/// Remove all background related components and resource.
///
/// # Schedule
/// Exit [crate::AppState::Loading].
pub fn despawn_background(
    mut commands: Commands,
    mut q_background: Query<Entity, With<BackgroundImage>>,
) {
    commands.entity(q_background.single_mut()).despawn();
}

/// UI builder extension for spawn wallpaper.
trait UiWallpaperBuilder {
    fn wallpaper(&mut self, image: UiImage) -> UiBuilder<'_, Entity>;
}

impl UiWallpaperBuilder for UiBuilder<'_, UiRoot> {
    fn wallpaper(&mut self, image: UiImage) -> UiBuilder<'_, Entity> {
        let mut builder = self.spawn(ImageBundle { image, ..default() });

        // the background image always has 16:9 aspect ratio.
        const ASPECT_RATIO: f32 = 16.0 / 9.0;

        builder
            .style()
            .align_self(AlignSelf::Center)
            .justify_self(JustifySelf::Center)
            .min_height(Val::Vh(100.0))
            .max_height(Val::Vw(ASPECT_RATIO.recip() * 100.0))
            .min_width(Val::Vw(100.0))
            .max_width(Val::Vh(ASPECT_RATIO * 100.0))
            .aspect_ratio(Some(ASPECT_RATIO));

        builder
    }
}
