use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct AudioSettings {}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {}
    }
}
