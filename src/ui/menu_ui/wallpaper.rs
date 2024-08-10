//! The wallpapers for [crate::AppState::InMenu] and
//! [crate::AppState::Loading] states.

use crate::ui::PrimaryCamera;
use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;
use sickle_ui::prelude::{generated::*, UiBuilder, UiBuilderExt, UiRoot};

/// A component for the wallpapers during [crate::AppState::InMenu]
/// and [crate::AppState::Loading].
///
/// Indicate an entity is wallpapers. Store all background
/// images to avoid loading time.
#[derive(Component, Clone, Default, Debug)]
pub struct Wallpapers {
    current: usize,
    images: Vec<Handle<Image>>,
}

impl Wallpapers {
    /// Create a new [Wallpapers] from a collection of images.
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
pub fn setup(
    mut commands: Commands,
    asserts: Res<AssetServer>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let camera = q_camera.single();

    let images = (0..2)
        .into_iter()
        .map(|i| asserts.load(format!("menu_ui/wallpaper/{}.png", i)))
        .collect();
    let background = Wallpapers::new(images);

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
/// In [crate::AppState::InMenu], need to have [Wallpapers] resource.
pub fn update(mut q_background: Query<(&mut UiImage, &mut Wallpapers), With<Wallpapers>>) {
    for (mut image, mut background) in q_background.iter_mut() {
        background.next();
        image.texture = background.image();
    }
}

/// Remove all background related components and resource.
///
/// # Schedule
/// Exit [crate::AppState::Loading].
pub fn cleanup(mut commands: Commands, mut q_background: Query<Entity, With<Wallpapers>>) {
    commands.entity(q_background.single_mut()).despawn();
}

/// UI builder extension for spawn wallpaper.
trait WallpaperUiBuilderExt {
    fn wallpaper(&mut self, image: UiImage) -> UiBuilder<'_, Entity>;
}

impl WallpaperUiBuilderExt for UiBuilder<'_, UiRoot> {
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
