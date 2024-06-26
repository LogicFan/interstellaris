use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Resource)]
pub struct ParsedInput {
    // the virtual pointer position
    pub pointer_x: f32,
    pub pointer_y: f32,

    // the pointer movement of last frame
    pub pointer_dx: f32,
    pub pointer_dy: f32,

    // the scroll movement of last frame
    pub scroll: f32,
}
