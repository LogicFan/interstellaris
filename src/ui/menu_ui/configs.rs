//! the configuration of ui
use bevy::{
    asset::Handle,
    color::{palettes::tailwind, Alpha, Color},
    prelude::{default, Resource},
    reflect::Reflect,
    text::Font,
};
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Configs {
    pub scale: f32,
    #[serde(skip)]
    pub text: TextConfigs,
    #[serde(skip)]
    pub menu: MenuConfigs,
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            scale: 1.0,
            text: Default::default(),
            menu: Default::default(),
        }
    }
}

/// Configuration related to text in UI.
#[derive(Clone, Debug, Reflect)]
pub struct TextConfigs {
    pub font: Handle<Font>,
    pub color: Color,
}

impl Default for TextConfigs {
    fn default() -> Self {
        Self {
            font: Default::default(),
            color: tailwind::NEUTRAL_300.into(),
        }
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct MenuConfigs {
    pub color_none: Color,
    pub color_focus: Color,
}

impl Default for MenuConfigs {
    fn default() -> Self {
        Self {
            color_none: tailwind::NEUTRAL_700.with_alpha(0.5).into(),
            color_focus: tailwind::NEUTRAL_700.into(),
        }
    }
}
