use super::galaxy::gen::{handle_galaxy_gen_task, init_galaxy_gen_task};
use crate::{states::LoadSource, AppState};
use bevy::prelude::*;
use rand::{thread_rng, RngCore};
use rand_pcg::Pcg64Mcg;

#[derive(SubStates, Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
            );
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
pub trait GenRng {
    /// Get the rng for each polity (a.k.a sovereign entity), each
    /// should have at least 2^32 non-overlapping range.
    fn polity_rng(&self, i: usize) -> Self;

    /// Get the rng for galaxy, each should have at least 2^64
    /// non-overlapping range.
    fn galaxy_rng(&self, i: usize) -> Self;

    /// Get the rng for planetary system generation. Should be called
    /// from a galaxy rng; each should have at lest 2^32 non-overlapping
    /// range.
    fn pln_sys_rng(&self, i: usize) -> Self;
}

impl GenRng for Pcg64Mcg {
    fn polity_rng(&self, i: usize) -> Self {
        let mut rng = self.clone();
        rng.advance(i as u128 * 2_u128.pow(32));
        rng
    }

    fn galaxy_rng(&self, i: usize) -> Pcg64Mcg {
        let mut rng = self.clone();
        // skip for the polity range.
        rng.advance(2_u128.pow(64));
        rng.advance(i as u128 * 2_u128.pow(32));
        rng
    }

    fn pln_sys_rng(&self, i: usize) -> Pcg64Mcg {
        let mut rng = self.clone();
        // skip the range reserved by galaxy itself.
        rng.advance(2_u128.pow(32));
        rng.advance(i as u128 * 2_u128.pow(32));
        rng
    }
}

pub fn default_rng() -> Pcg64Mcg {
    let mut random_seed: u128 = thread_rng().next_u64() as u128;
    random_seed = random_seed << 64;
    random_seed += thread_rng().next_u64() as u128;

    Pcg64Mcg::new(random_seed)
}
