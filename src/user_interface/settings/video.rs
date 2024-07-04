use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Resource)]
pub struct VideoSettings {
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {}
    }
}
