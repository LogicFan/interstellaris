use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct InputSettings {
    pub mouse_motion_sensitivity: f32,
    pub mouse_scroll_sensitivity: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            mouse_motion_sensitivity: 1.0,
            mouse_scroll_sensitivity: 1.0,
        }
    }
}
