use super::{args::GalaxyGenArgs, GameMapGenArgs};
use bevy::{prelude::*, tasks::Task};
use float_ord::FloatOrd;
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;
use std::collections::BTreeMap;
use voronoice::{self, VoronoiCell};

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PlanetarySystemArgs {
    pub transform: Vec3,
    pub mass: f32,
    // TODO: add remnant and nebula
}

#[derive(Component, Debug)]
pub struct PlanetarySystemGenTask(Task<Vec<PlanetarySystemArgs>>);

pub fn init_galaxy(
    mut commands: Commands,
    asset: Res<AssetServer>,
    q_args: Query<&GameMapGenArgs>,
) {
    let args = q_args.single();

    gen_planetary_systems(args.rng.clone(), args.galaxy);

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
    let radius = 0.5 * (args.size as f32 / args.density).sqrt();

    let transforms = random_transforms(&mut rng, radius, args.size as usize);
    let masses = compute_mass(&transforms, radius, args.density);

    info!("{:?}", masses);

    return Vec::new();
}

fn random_transforms(rng: &mut Pcg64Mcg, radius: f32, size: usize) -> Vec<Vec3> {
    let mut transforms = BTreeMap::<FloatOrd<f32>, Vec3>::new();

    let height = 16.0;
    let radius_distr = Uniform::new_inclusive(-radius, radius);
    let height_distr = Uniform::new_inclusive(-height, height);

    let iterations = size * 2;

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

        if transforms.len() == size as usize {
            break 'outer;
        }
    }

    transforms.values().copied().collect()
}

fn compute_mass(transforms: &Vec<Vec3>, radius: f32, density: f32) -> Vec<f32> {
    let sites: Vec<_> = transforms
        .iter()
        .map(|e| voronoice::Point {
            x: e.x as f64,
            y: e.y as f64,
        })
        .collect();

    let voronoi = voronoice::VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(voronoice::BoundingBox::new_centered_square(
            (radius * 2.0) as f64,
        ))
        .build()
        .unwrap();

    fn area_of_cell(cell: VoronoiCell) -> f32 {
        let mut area = 0.0;
        let vertices = cell.iter_vertices().collect::<Vec<_>>();
        if vertices.len() >= 2 {
            let p1 = cell.site_position();
            for i in 0..vertices.len() {
                let p2 = vertices[i];
                let p3 = vertices[(i + 1) % vertices.len()];

                let a = 0.5
                    * (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)).abs();

                area += a;
            }
        }

        area as f32
    }

    voronoi
        .iter_cells()
        .map(area_of_cell)
        .map(|x| x * density)
        .collect::<Vec<_>>()
}
