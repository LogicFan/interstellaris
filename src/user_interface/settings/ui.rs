use bevy::prelude::*;

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
            text_color: Color::hex("#d4d4d4").unwrap(),

            background_color_1: Color::hex("#262626").unwrap().with_a(0.6),
            background_color_2: Color::hex("#262626").unwrap().with_a(1.0),
        }
    }
}
