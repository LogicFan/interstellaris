//! This module contain the setting related to input and ui.

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct InputSettings {
    pub mouse_motion_sensitivity: f64,
    pub mouse_scroll_sensitivity: f64,
}