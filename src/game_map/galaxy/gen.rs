use super::Galaxy;
use crate::game_map::gen::GenState;
use crate::game_map::planetary_system::gen::PlnSysGenParams;
use crate::game_map::planetary_system::PlanetarySystem;
use crate::game_map::BoundingSize;
use crate::utils::beta_params;
use crate::utils::ObjectId;
use crate::utils::{default_rng, RngExt};
use bevy::math::FloatOrd;
use bevy::prelude::*;
use bevy::tasks::block_on;
use bevy::tasks::futures_lite::future;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::Task;
use rand_distr::Beta;
use rand_distr::Distribution;
use rand_distr::Uniform;
use rand_pcg::Pcg64Mcg;
use std::collections::BTreeMap;

/// Used for game map generation. This is the initial parameters for
/// the galaxy random generation.
#[derive(Component, Clone, Debug)]
pub struct GalaxyGenParams {
    /// the random generator for this galaxy.
    pub rng: Pcg64Mcg,
    /// the number of planetary system in the galaxy
    pub size: usize,
    /// the density of the planetary system, in terms of 1/ly^2
    pub density: f32,
    // TODO: add star stages and nebula
}

impl Default for GalaxyGenParams {
    fn default() -> Self {
        Self {
            rng: default_rng(),
            size: 8192,
            density: 0.0625,
        }
    }
}

impl GalaxyGenParams {
    fn radius(&self) -> f32 {
        0.5 * (self.size as f32 / self.density).sqrt()
    }

    fn height(&self) -> f32 {
        12.0
    }

    fn delta(&self) -> f32 {
        1.0
    }
}

#[derive(Component, Debug)]
pub struct GenTask(Task<Vec<PlnSysGenParams>>);

pub fn init_galaxy_gen_task(
    mut commands: Commands,
    q_galaxy: Query<(Entity, &GalaxyGenParams), With<Galaxy>>,
) {
    let task_pool = AsyncComputeTaskPool::get();

    let (entity, params) = q_galaxy.single();
    let params = params.clone();
    let rng = params.rng.clone();

    let task = task_pool.spawn(async move { new_planetary_systems(params, rng) });

    commands.entity(entity).insert(GenTask(task));
}

pub fn handle_galaxy_gen_task(
    mut commands: Commands,
    mut gen_state: ResMut<NextState<GenState>>,
    mut q_galaxy: Query<(Entity, &mut GenTask, &GalaxyGenParams), With<Galaxy>>,
) {
    let mut count = 0;

    for (entity, mut task, params) in q_galaxy.iter_mut() {
        count += 1;

        if let Some(planetary_systems) = block_on(future::poll_once(&mut task.0)) {
            let xyz = Vec3::new(params.radius(), params.radius(), params.height());

            commands
                .entity(entity)
                .remove::<GalaxyGenParams>()
                .remove::<GenTask>()
                .insert(BoundingSize(xyz))
                .insert(ObjectId::default());

            for params in planetary_systems {
                commands.spawn((PlanetarySystem, params));
            }
        }
    }

    if count == 0 {
        gen_state.set(GenState::InitPlnSys)
    }
}

fn new_planetary_systems(galaxy: GalaxyGenParams, mut rng: Pcg64Mcg) -> Vec<PlnSysGenParams> {
    let mut planetary_systems = Vec::with_capacity(galaxy.size);

    // assign positions
    {
        let mut x_range = BTreeMap::<FloatOrd, Vec3>::new();

        let radius_distr = Uniform::new_inclusive(-galaxy.radius(), galaxy.radius());
        let height_distr = {
            const SIGMA: f32 = 0.05;
            let (alpha, beta) = beta_params(0.5, SIGMA);
            Beta::new(alpha, beta)
                .unwrap()
                .map(|x| (x - 0.5) * 2.0 * galaxy.height())
        };

        // double the number of iterations to compensate for potential
        // rejected position.
        'outer: for _ in 0..(galaxy.size * 2) {
            let x = radius_distr.sample(&mut rng);
            let y = radius_distr.sample(&mut rng);
            let z = height_distr.sample(&mut rng);
            let candidate = Vec3::new(x, y, z);

            // reject this value if it is to close to a existing one.
            for (_, other) in
                x_range.range(FloatOrd(x - galaxy.delta())..=FloatOrd(x + galaxy.delta()))
            {
                // we want to ensure xy plane are sufficiently separate for the
                // mini map
                if candidate.xy().distance(other.xy()) < galaxy.delta() * 2.0 {
                    continue 'outer;
                }
            }

            x_range.insert(FloatOrd(x), candidate);

            if x_range.len() == galaxy.size as usize {
                break 'outer;
            }
        }

        x_range.values().enumerate().for_each(|(i, position)| {
            let mut rng = galaxy.rng.clone();
            rng.advance32(i);
            planetary_systems.push(PlnSysGenParams {
                rng,
                position: *position,
                mass: 1.0,
            });
        });
    }

    planetary_systems
}
