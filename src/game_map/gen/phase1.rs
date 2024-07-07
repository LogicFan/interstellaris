use super::{args::GalaxyGenArgs, GameMapGenArgs};
use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;

pub fn init_galaxy(
    mut commands: Commands,
    asset: Res<AssetServer>,
    q_args: Query<&GameMapGenArgs>,
) {
    let args = q_args.single();
    let mut rng = args.rng.clone();

    let positions = gen_positions(&mut rng, args.galaxy);

    let mesh = asset.add(Cuboid::new(0.5, 0.5, 0.5).into());
    let material = asset.add(StandardMaterial {
        emissive: LinearRgba::new(1000.0, 1000.0, 1000.0, 1.0),
        ..default()
    });

    use crate::planetary_system::*;
    use bevy_mod_picking::*;

    for position in positions {
        for _ in 0..100 {
            commands.spawn((
                PlanetarySystemBundle {
                    transform: Transform::from_translation(position),
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
    }
}

fn gen_positions(rng: &mut Pcg64Mcg, args: GalaxyGenArgs) -> Vec<Vec3> {
    let mut res = Vec::<Vec3>::new();

    let radius = 0.5 * (args.size as f32 / args.density).sqrt();
    let height = 16.0;

    let iterations = args.size * 2;
    let radius_distr = Uniform::new_inclusive(-radius, radius);
    let height_distr = Uniform::new_inclusive(-height, height);
    'outer: for _ in 0..iterations {
        let x = radius_distr.sample(rng);
        let y = radius_distr.sample(rng);
        let z = height_distr.sample(rng);

        // TODO: optimize the efficiency here
        for other in res.iter() {
            if ((other.x - x).powf(2.0) + (other.y - y).powf(2.0)).sqrt() < 0.1 {
                continue 'outer;
            }
        }

        res.push(Vec3::new(x, y, z));

        if res.len() == args.size as usize {
            break 'outer;
        }
    }

    res
}
