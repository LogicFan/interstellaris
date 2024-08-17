//! For parsed inputs

use bevy::{math::Vec2, prelude::Resource, reflect::Reflect};

/// A resource to store input of the mouse.
#[derive(Debug, Default, Clone, Copy, Resource, Reflect)]
pub struct MouseMotion {
    /// The viewport size which mouse pointer is currently on.
    pub viewport: Vec2,
    /// The current position of the mouse pointer.
    pub position: Vec2,
    /// The mouse pointer position of the last frame.
    pub last_position: Vec2,
    /// The mouse pointer motion from the last frame.
    pub motion: Vec2,
    /// Indicate if the mouse pointer is on the viewport border. `x` indicate the
    /// horizontal axis, -1 is on the left and 1 is on the right; `y` indicate
    /// the vertical axis, -1 is the top and 1 is the bottom. 0 always indicate
    /// not on the border for the respected axis.
    pub on_border: Vec2,
}

impl MouseMotion {
    /// Prepare for the new frame data.
    pub(super) fn refresh(&mut self, viewport: Vec2) {
        self.viewport = viewport;
        self.last_position = self.position;
        self.motion = Vec2::ZERO;
    }
    /// Set the mouse pointer movement. Return the actual
    /// delta after the constraint.
    pub(super) fn add_delta(&mut self, delta: Vec2) -> Vec2 {
        self.motion += delta;
        let new_delta = delta.clamp(-self.position, self.viewport - self.position);
        self.position += new_delta;
        new_delta
    }
    /// Compute additional fields for motion. Must be called at the end of input update
    pub(super) fn post_update(&mut self) {
        let epsilon = f32::EPSILON * self.viewport;
        self.on_border.x = f32::from(self.position.x >= self.viewport.x - epsilon.x)
            - f32::from(self.position.x <= 0.0 + epsilon.x);
        self.on_border.y = f32::from(self.position.y >= self.viewport.y - epsilon.y)
            - f32::from(self.position.y <= 0.0 + epsilon.y);
    }
}

/// A resource to store input of the mouse.
#[derive(Debug, Default, Clone, Copy, Resource, Reflect)]
pub struct MouseWheel {
    /// The scroll of mouse from the last frame. Including both vertical scroll
    /// and horizontal scroll.
    pub scroll: Vec2,
}

impl MouseWheel {
    /// Prepare for the new frame data.
    pub(super) fn refresh(&mut self) {
        self.scroll = Vec2::ZERO;
    }
    /// Set the mouse wheel scroll.
    pub(super) fn add_delta(&mut self, delta: Vec2) {
        self.scroll += delta;
    }
}

pub struct Button {}

impl Button {
    pub(super) fn press(&mut self) {}
    pub(super) fn release(&mut self) {}

    pub fn pressed(&self) -> bool {
        todo!()
    }
    pub fn clicked(&self) -> bool {
        todo!()
    }
    pub fn double_clicked(&self) -> bool {
        todo!()
    }

    pub fn click_count(&self) -> bool {
        todo!()
    }
}
