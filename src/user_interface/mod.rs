mod main_camera;
mod pointer;

pub use main_camera::init_main_camera;
pub use pointer::init_virtual_pointer;

use bevy::prelude::*;


#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct ControlSettings {
    mouse_motion_sensitivity: f64,
    mouse_scroll_sensitivity: f64,
}

#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct ParsedInput {
    // the position of virtual pointer
    pub x: f32,
    pub y: f32,

    // the relative movement of last frame
    pub dx: f32,
    pub dy: f32,
}
