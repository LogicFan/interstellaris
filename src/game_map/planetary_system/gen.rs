use super::PlanetarySystem;
use crate::{
    game_map::{
        galaxy::{Galaxy, GalaxySize},
        planetary_system::{PlanetarySystemBundle, VPlanetarySystemBundle},
    },
    states::AppState,
    ui::camera::MotionMode,
};
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand_pcg::Pcg64Mcg;
use std::f32::consts::PI;

#[derive(Component, Debug, Clone)]
pub struct PlnSysGenParams {
    pub rng: Pcg64Mcg,
    pub position: Vec3,
    pub mass: f32,
    // TODO: add remnant and nebula
}

pub fn spawn_planetary_systems(
    mut commands: Commands,
    asset: Res<AssetServer>,
    q_galaxy: Query<&GalaxySize, With<Galaxy>>,
    q_pln_sys: Query<&PlnSysGenParams, With<PlanetarySystem>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut cam_mo: ResMut<MotionMode>,
) {
    let mesh = asset.add(Sphere::default().mesh().ico(16).unwrap());
    let material = asset.add(StandardMaterial {
        emissive: LinearRgba::new(1000.0, 500.0, 500.0, 1.0),
        ..default()
    });

    for planetary_system in q_pln_sys.iter() {
        commands.spawn((
            PlanetarySystemBundle {
                transform: Transform::from_translation(planetary_system.position)
                    .with_scale(Vec3::ONE * planetary_system.mass * 0.1),
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

    *cam_mo = MotionMode::FreeMotion {
        min_h: 32.0,
        max_h: 100.0,
        max_θ: PI / 3.0,
        max_r: q_galaxy.single().max.x * 2.0,
    };

    info!("123");

    app_state.set(AppState::InGame);
}
