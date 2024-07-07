//! This module contain the setting related to input and ui.

mod audio;
mod input;
mod ui;
mod video;

pub use audio::AudioSettings;
pub use input::InputSettings;
pub use ui::UiSettings;
pub use video::VideoSettings;

use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioSettings::default())
            .insert_resource(InputSettings::default())
            .insert_resource(UiSettings::default())
            .insert_resource(VideoSettings::default());
    }
}
