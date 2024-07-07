use super::UiSettings;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use sickle_ui::prelude::{generated::*, UiBuilder, UiContainerExt};

pub trait MenuUiBuilderExt {
    fn text_button(
        &mut self,
        settings: &UiSettings,
        text: &str,
        scale: f32,
    ) -> UiBuilder<'_, Entity>;

    fn large_text_button(&mut self, settings: &UiSettings, text: &str) -> UiBuilder<'_, Entity> {
        self.text_button(settings, text, 2.0)
    }
}

impl MenuUiBuilderExt for UiBuilder<'_, Entity> {
    fn text_button(
        &mut self,
        settings: &UiSettings,
        text: &str,
        scale: f32,
    ) -> UiBuilder<'_, Entity> {
        let ui_scale = settings.ui_scale * scale;
        let text_color = settings.text_color;
        let bg_color_none = settings.bg_color_none;
        let bg_color_focus = settings.bg_color_focus;

        let text_style = TextStyle {
            font: settings.font.clone(),
            font_size: 16.0 * ui_scale,
            color: text_color,
        };

        let mut builder = self.container(ButtonBundle::default(), |parent| {
            parent
                .spawn(
                    TextBundle::from_section(text, text_style)
                        .with_text_justify(JustifyText::Center),
                )
                .insert(Pickable::IGNORE);
        });

        builder
            .insert(On::<Pointer<Out>>::target_component_mut::<BackgroundColor>(
                move |_, bg_color| {
                    bg_color.0 = bg_color_none;
                },
            ))
            .insert(
                On::<Pointer<Over>>::target_component_mut::<BackgroundColor>(move |_, bg_color| {
                    bg_color.0 = bg_color_focus;
                }),
            );

        builder
            .style()
            .align_content(AlignContent::Center)
            .justify_content(JustifyContent::Center)
            .padding(UiRect::vertical(Val::Px(4.0)))
            .border(UiRect::all(Val::Px(1.0)))
            .width(Val::Px(100.0 * ui_scale))
            .border_color(settings.text_color)
            .background_color(settings.bg_color_none);

        builder
    }
}
