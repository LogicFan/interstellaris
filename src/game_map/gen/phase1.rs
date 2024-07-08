use std::{collections::BTreeMap, ops::RangeInclusive};

use super::{args::GalaxyGenArgs, GameMapGenArgs};
use bevy::{prelude::*, tasks::Task};
use float_ord::FloatOrd;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlanetarySystemArgs {
    pub transform: Vec3,
    pub mass: f32,
}

#[derive(Component, Debug)]
pub struct PlanetarySystemGenTask(Task<Vec<PlanetarySystemArgs>>);

pub fn init_galaxy(
    mut commands: Commands,
    asset: Res<AssetServer>,
    q_args: Query<&GameMapGenArgs>,
) {
    // let args = q_args.single();
    // let mut rng = args.rng.clone();

    // let positions = random_positions(&mut rng, args.galaxy);

    // let mesh = asset.add(Cuboid::new(0.5, 0.5, 0.5).into());
    // let material = asset.add(StandardMaterial {
    //     emissive: LinearRgba::new(1000.0, 1000.0, 1000.0, 1.0),
    //     ..default()
    // });

    // use crate::planetary_system::*;
    // use bevy_mod_picking::*;

    // for position in positions {
    //     commands.spawn((
    //         PlanetarySystemBundle {
    //             transform: Transform::from_translation(position),
    //             ..default()
    //         },
    //         VPlanetarySystemBundle {
    //             mesh: mesh.clone(),
    //             material: material.clone(),
    //             ..default()
    //         },
    //         PickableBundle::default(),
    //     ));
    // }
}

fn gen_planetary_systems(rng0: Pcg64Mcg, args: GalaxyGenArgs) -> Vec<PlanetarySystemArgs> {
    let mut rng = rng0.clone();
    let transforms = random_transforms(&mut rng, &args);

    todo!()
}

fn random_transforms(rng: &mut Pcg64Mcg, args: &GalaxyGenArgs) -> Vec<Vec3> {
    let mut transforms = BTreeMap::<FloatOrd<f32>, Vec3>::new();

    let radius = 0.5 * (args.size as f32 / args.density).sqrt();
    let height = 16.0;
    let radius_distr = Uniform::new_inclusive(-radius, radius);
    let height_distr = Uniform::new_inclusive(-height, height);

    let iterations = args.size * 2;

    'outer: for _ in 0..iterations {
        const DELTA: f32 = 0.21;

        let x = radius_distr.sample(rng);
        let y = radius_distr.sample(rng);
        let z = height_distr.sample(rng);
        let candidate = Vec3::new(x, y, z);

        // reject this value if it is to close to a existing one.
        for (_, other) in transforms.range(FloatOrd(x - DELTA)..=FloatOrd(x + DELTA)) {
            if candidate.distance(*other) < DELTA {
                continue 'outer;
            }
        }

        transforms.insert(FloatOrd(x), candidate);

        if transforms.len() == args.size as usize {
            break 'outer;
        }
    }

    transforms.values().copied().collect()
}
