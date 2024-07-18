use bevy::prelude::*;
use rand_pcg::Pcg64Mcg;

#[derive(Component, Debug, Clone)]
pub struct PlnSysGenParams {
    pub rng: Pcg64Mcg,
    pub position: Vec3,
    pub mass: f32,
    // TODO: add remnant and nebula
}
