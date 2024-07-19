use super::{
    galaxy::gen::{handle_galaxy_gen_task, init_galaxy_gen_task},
    planetary_system::gen::spawn_planetary_systems,
};
use crate::{states::LoadSource, AppState};
use bevy::prelude::*;
use rand::{thread_rng, RngCore};
use rand_pcg::Pcg64Mcg;

#[derive(SubStates, Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[source(AppState = AppState::Loading(LoadSource::Generation))]
pub enum GenState {
    #[default]
    InitGalaxy,
    InitPlnSys,
}

/// The plugin for game map generation.
pub struct GampMapGenPlugin;

impl Plugin for GampMapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<GenState>()
            .add_systems(OnEnter(GenState::InitGalaxy), init_galaxy_gen_task)
            .add_systems(
                Update,
                handle_galaxy_gen_task.run_if(in_state(GenState::InitGalaxy)),
            )
            .add_systems(OnEnter(GenState::InitPlnSys), spawn_planetary_systems);
    }
}

/// a trait for getting RNGs deterministically for each
/// part of map generation task.
///
/// - the first `[0, 2^64)` is for empires generation
/// - for each galaxy, they will occupy 2^64 numbers.
/// Within this range, the first 2^32 numbers will be used for galaxy
/// itself, and the following will be used for planetary systems.
/// - for each planetary system, it will use 2^32 numbers.
pub trait RngExt {
    /// advance i * 2^16 steps;
    fn advance16(&mut self, i: usize);

    /// advance i * 2^32 steps;
    fn advance32(&mut self, i: usize);

    /// advance i * 2^48 steps;
    fn advance48(&mut self, i: usize);

    /// advance i * 2^64 steps;
    fn advance64(&mut self, i: usize);
}

impl RngExt for Pcg64Mcg {
    fn advance16(&mut self, i: usize) {
        self.advance(i as u128 * 2_u128.pow(16));
    }

    fn advance32(&mut self, i: usize) {
        self.advance(i as u128 * 2_u128.pow(32));
    }

    fn advance48(&mut self, i: usize) {
        self.advance(i as u128 * 2_u128.pow(48));
    }

    fn advance64(&mut self, i: usize) {
        self.advance(i as u128 * 2_u128.pow(64));
    }
}

pub fn default_rng() -> Pcg64Mcg {
    let mut random_seed: u128 = thread_rng().next_u64() as u128;
    random_seed = random_seed << 64;
    random_seed += thread_rng().next_u64() as u128;

    Pcg64Mcg::new(random_seed)
}
