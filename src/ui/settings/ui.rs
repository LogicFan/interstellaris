use bevy::{color::palettes::tailwind, prelude::*};

#[derive(Debug, Clone, Resource)]
pub struct UiSettings {
    /// 1.0 is the normal size
    pub ui_scale: f32,

    #[allow(unused)]
    pub font: Handle<Font>,
    pub text_color: Color,

    pub bg_color_none: Color,
    pub bg_color_focus: Color,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            ui_scale: 1.0,

            font: Default::default(),
            text_color: tailwind::NEUTRAL_300.into(),

            bg_color_none: tailwind::NEUTRAL_700.into(),
            bg_color_focus: tailwind::NEUTRAL_800.into(),
        }
    }
}
