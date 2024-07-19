// TODO: rework on this....

pub mod primary_camera;

use bevy::prelude::*;

pub use primary_camera::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum CameraMotionSystemSet {
    PrimaryCamera,
}
