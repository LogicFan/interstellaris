use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Resource)]
pub struct AudioSettings {}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {}
    }
}