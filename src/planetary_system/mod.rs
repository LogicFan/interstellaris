pub mod api;

use crate::utils::{ObjectId, ObjectRef};
use bevy::math::primitives::Cuboid;
use bevy::prelude::*;
use bevy_mod_picking::{focus::PickingInteraction, PickableBundle};
use rand::{thread_rng, Rng};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PlanetarySystem;

#[derive(Debug, Default, Clone, Component)]
pub struct Planets(pub Vec<ObjectRef>);

#[derive(Debug, Default, Clone, Bundle)]
pub struct LPlanetarySystemBundle {
    pub marker: PlanetarySystem,
    pub id: ObjectId,
    pub transform: Transform,
    pub planets: Planets,
}

#[derive(Debug, Default, Bundle, Clone)]
pub struct VPlanetarySystemBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

pub fn init_planetary_system(mut commands: Commands, asset: Res<AssetServer>) {
    let mut rng = thread_rng();
    let mesh = asset.add(Cuboid::new(0.5, 0.5, 0.5).into());
    let material = asset.add(StandardMaterial {
        emissive: Color::rgb_linear(10000.0, 10000.0, 10000.0),
        ..default()
    });
    for _ in 0..100 {
        let x = (rng.gen::<f32>() - 0.5) * 60.0;
        let y = (rng.gen::<f32>() - 0.5) * 60.0;
        let z = (rng.gen::<f32>() - 0.5) * 1.0;
        commands.spawn((
            LPlanetarySystemBundle {
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            VPlanetarySystemBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                ..default()
            },
            // PickableBundle::default(),
        ));
    }
}

fn update_button_colors(
    mut buttons: Query<(Option<&PickingInteraction>, &mut BackgroundColor), With<Button>>,
) {
    for (interaction, mut button_color) in &mut buttons {
        *button_color = match interaction {
            Some(PickingInteraction::Pressed) => Color::rgb(0.35, 0.75, 0.35),
            Some(PickingInteraction::Hovered) => Color::rgb(0.25, 0.25, 0.25),
            Some(PickingInteraction::None) | None => Color::rgb(0.15, 0.15, 0.15),
        }
        .into();
    }
}
