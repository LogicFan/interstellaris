use super::{MenuState, PrevPageStack, UiSettings};
use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, On, Out, Over, Pickable, Pointer};
use sickle_ui::prelude::{generated::*, UiBuilder, UiColumnExt, UiContainerExt, UiRoot, UiRowExt};

pub trait MenuUiBuilderExt0: UiColumnExt + UiRowExt + UiContainerExt {
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

    fn large_text_button(&mut self, settings: &UiSettings, text: &str) -> UiBuilder<'_, Entity> {
        self.text_button(settings, text, 2.0)
    }

    fn medium_text_button(&mut self, settings: &UiSettings, text: &str) -> UiBuilder<'_, Entity> {
        self.text_button(settings, text, 1.5)
    }

    fn sub_menu_container<Marker>(
        &mut self,
        settings: &UiSettings,
        confirm_text: &str,
        confirm_action: impl IntoSystem<(), (), Marker>,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity> {
        let mut builder = self.column(|column| {
            column
                .container(NodeBundle::default(), |parent| {
                    spawn_children(parent);
                })
                .style()
                .border(UiRect::all(Val::Px(1.0)))
                .justify_content(JustifyContent::Center)
                .align_content(AlignContent::Center)
                .border_color(settings.text_color)
                .background_color(settings.bg_color_none)
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0));

            column
                .row(|row| {
                    row.medium_text_button(settings, "Back")
                        .insert(On::<Pointer<Click>>::run(
                            |mut state: ResMut<NextState<MenuState>>,
                             mut stack: ResMut<PrevPageStack>| {
                                if let Some(prev_page) = stack.0.pop() {
                                    state.set(prev_page)
                                }
                            },
                        ));

                    row.medium_text_button(settings, confirm_text)
                        .insert(On::<Pointer<Click>>::run(confirm_action));
                })
                .style()
                .width(Val::Percent(100.0))
                .justify_content(JustifyContent::SpaceBetween);
        });

        builder
            .style()
            .row_gap(Val::Px(16.0))
            .width(Val::Vw(100.0))
            .height(Val::Vh(100.0))
            .padding(UiRect::all(Val::Px(32.0)));

        builder
    }
    // fn settings_slider(&mut self) -> UiBuilder<'_, Entity> {
    //     self.column()
    // }

    // fn settings_text(&mut self) -> UiBuilder<'_, Entity> {
    //     self
    // }

    // fn settings_options(&mut self) -> UiBuilder<'_, Entity> {
    //     self
    // }

    // fn settings_dropdown(&mut self) -> UiBuilder<'_, Entity> {
    //     self
    // }
}

impl MenuUiBuilderExt0 for UiBuilder<'_, UiRoot> {}
impl MenuUiBuilderExt0 for UiBuilder<'_, Entity> {}
