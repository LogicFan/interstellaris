//! This module controls the behavior of the cursor.

use bevy::prelude::Query;
use bevy::prelude::With;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;
use bevy::window::Window;

pub fn lock_cursor(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>
) {
    let mut window = q_window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined;
}

pub fn release_cursor(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>
) {
    let mut window = q_window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::None;
}
