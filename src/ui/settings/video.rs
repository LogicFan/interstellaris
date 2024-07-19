use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct VideoSettings {}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {}
    }
}
