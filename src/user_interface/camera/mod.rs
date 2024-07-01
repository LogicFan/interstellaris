mod primary_camera;

use bevy::prelude::*;

pub use primary_camera::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraMotionSystemSet {
    PrimaryCamera
}
