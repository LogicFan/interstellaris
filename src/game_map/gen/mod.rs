mod args;
mod phase1;

pub use phase1::init_galaxy;

use crate::AppState;
use args::GalaxyGenArgs;
use bevy::prelude::*;
use rand::{thread_rng, RngCore};
use rand_pcg::Pcg64Mcg;

#[derive(Component, Debug, Clone)]
pub struct GameMapGenArgs {
    pub rng: Pcg64Mcg,
    pub galaxy: GalaxyGenArgs,
}

impl Default for GameMapGenArgs {
    fn default() -> Self {
        let mut random_seed: u128 = thread_rng().next_u64() as u128;
        random_seed = random_seed << 64;
        random_seed += thread_rng().next_u64() as u128;

        Self {
            rng: Pcg64Mcg::new(random_seed),
            galaxy: default(),
        }
    }
}

pub struct GampMapGenPlugin;

impl Plugin for GampMapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), init_galaxy);
    }
}
