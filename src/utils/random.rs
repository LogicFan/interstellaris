//! Extension methods for random number generators.

use rand::{thread_rng, RngCore};
use rand_pcg::Pcg64Mcg;

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

pub fn beta_params(mu: f32, sigma: f32) -> (f32, f32) {
    let n = mu * (1.0 - mu) / sigma.powi(2);
    let alpha = mu * n;
    let beta = (1.0 - mu) * n;
    (alpha, beta)
}
