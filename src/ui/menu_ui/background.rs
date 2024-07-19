//! The background image for [crate::AppState::InMenu] and
//! [crate::AppState::Loading]

use crate::ui::{menu_ui::ui_builder_ext::MenuUiBuilderExt0, PrimaryCamera};
use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;
use sickle_ui::prelude::{generated::*, UiBuilderExt, UiRoot};

/// A marker component for background image.
#[derive(Component, Copy, Clone, Default, Debug)]
pub struct MenuBackground;

/// A resource to store all background images.
///
/// This should exists for the entire [crate::AppState::InMenu]
/// and [crate::AppState::Loading] to avoid additional resource
/// loading time when displayed image are changed.
#[derive(Resource, Clone, Default, Debug)]
pub struct BackgroundImages {
    id: usize,
    images: Vec<Handle<Image>>,
}

impl BackgroundImages {
    /// Create a new [BackgroundImages] from a collection of images.
    fn new(images: Vec<Handle<Image>>) -> Self {
        Self { id: 0, images }
    }

    /// Get the [Handle] of current background image.
    fn image(&self) -> Handle<Image> {
        self.images[self.id].clone()
    }

    /// Change to next image.
    fn next(&mut self) {
        self.id = (self.id + 1) % self.images.len();
    }
}

/// Load all the background images for the menu.
///
/// # Schedule
/// Enter [crate::AppState::InMenu]
pub fn load_background(mut commands: Commands, asserts: Res<AssetServer>) {
    let images = (0..2)
        .into_iter()
        .map(|i| asserts.load(format!("background/{}.png", i)))
        .collect();

    let resource = BackgroundImages::new(images);
    commands.insert_resource(resource);
}

/// Spawn the background for the menu.
///
/// # Schedule
/// Enter [crate::AppState::InMenu], need to have [BackgroundImages] resource.
pub fn spawn_background(
    mut commands: Commands,
    background: Res<BackgroundImages>,
    q_camera: Query<Entity, With<PrimaryCamera>>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .background_image(UiImage {
            texture: background.image(),
            ..default()
        })
        .insert(TargetCamera(camera))
        .insert(Name::new("Menu Background"))
        .insert(MenuBackground)
        .insert(Pickable::IGNORE)
        .style()
        .z_index(ZIndex::Global(-64));
}

/// Update background image to next image.
///
/// # Schedule
/// In [crate::AppState::InMenu], need to have [BackgroundImages] resource.
pub fn update_background(
    mut background: ResMut<BackgroundImages>,
    mut q_background: Query<&mut UiImage, With<MenuBackground>>,
) {
    background.next();
    q_background.single_mut().texture = background.image();
}

/// Remove all background related components and resource.
///
/// # Schedule
/// Exit [crate::AppState::Loading].
pub fn clean_background(
    mut commands: Commands,
    mut q_background: Query<Entity, With<MenuBackground>>,
) {
    commands.remove_resource::<BackgroundImages>();
    commands.entity(q_background.single_mut()).despawn();
}
