use super::PlanetarySystem;
use crate::{
    game_map::{
        galaxy::Galaxy,
        planetary_system::{PlanetarySystemBundle, VPlanetarySystemBundle},
        BoundingSize,
    },
    states::AppState,
};
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand_pcg::Pcg64Mcg;

#[derive(Component, Clone, Debug)]
pub struct PlnSysGenParams {
    pub rng: Pcg64Mcg,
    pub position: Vec3,
    pub mass: f32,
    // TODO: add remnant and nebula
}

pub fn spawn_planetary_systems(
    mut commands: Commands,
    asset: Res<AssetServer>,
    q_pln_sys: Query<&PlnSysGenParams, With<PlanetarySystem>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let mesh = asset.add(Sphere::default().mesh().ico(16).unwrap());

    for planetary_system in q_pln_sys.iter() {
        // let r = (planetary_system.mass - min_mass) / (max_mass - min_mass);
        // let b = 1.0 - r;

        let material = asset.add(StandardMaterial {
            base_color: LinearRgba::new(1000.0, 1000.0, 2000.0, 1.0).into(),
            ..default()
        });

        commands.spawn((
            PlanetarySystemBundle {
                transform: Transform::from_translation(planetary_system.position)
                    .with_scale(Vec3::ONE * planetary_system.mass * 0.2),
                ..default()
            },
            VPlanetarySystemBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                ..default()
            },
            PickableBundle::default(),
        ));
    }

    commands.spawn(PbrBundle {
        mesh: asset.add(Cuboid::new(10.0, 10.0, 0.1).mesh().build()),
        material: asset.add(StandardMaterial {
            base_color: LinearRgba::new(1.0, 1.0, 1.0, 1.0).into(),
            ..default()
        }),
        ..default()
    });

    app_state.set(AppState::InGame);
}
