use bevy::{color::palettes::tailwind, prelude::*};

#[derive(Debug, Clone, Resource)]
pub struct UiSettings {
    /// 1.0 is the normal size
    pub ui_scale: f32,

    #[allow(unused)]
    pub font: Handle<Font>,
    pub text_color: Color,

    pub background_color_1: Color,
    pub background_color_2: Color,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            ui_scale: 1.0,

            font: Default::default(),
            text_color: tailwind::NEUTRAL_300.into(),

            background_color_1: tailwind::NEUTRAL_800.with_alpha(0.6).into(),
            background_color_2: tailwind::NEUTRAL_800.with_alpha(1.0).into(),
        }
    }
}
