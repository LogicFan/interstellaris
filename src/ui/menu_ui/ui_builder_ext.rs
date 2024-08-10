use super::{configs::Configs, MenuState, PrevPageStack};
use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, On, Out, Over, Pickable, Pointer};
use sickle_ui::prelude::{generated::*, UiBuilder, UiColumnExt, UiContainerExt, UiRoot, UiRowExt};

pub trait TextButtonUiBuilderExt: UiContainerExt {
    fn text_button<T>(
        &mut self,
        cfg: &Configs,
        scale: f32,
        content: (&str, impl IntoSystem<(), (), T>),
    ) -> UiBuilder<'_, Entity> {
        let ui_scale = cfg.scale * scale;
        let text_color = cfg.text.color;

        let text_style = TextStyle {
            font: cfg.text.font.clone(),
            font_size: 16.0 * ui_scale,
            color: text_color,
        };

        let mut builder = self.container(ButtonBundle::default(), |parent| {
            parent
                .spawn(
                    TextBundle::from_section(content.0, text_style)
                        .with_text_justify(JustifyText::Center),
                )
                .insert(Pickable::IGNORE);
        });

        let ui_color_none = cfg.menu.color_none;
        let ui_color_focus = cfg.menu.color_focus;

        builder
            .insert(On::<Pointer<Out>>::target_component_mut::<BackgroundColor>(
                move |_, bg_color| {
                    bg_color.0 = ui_color_none;
                },
            ))
            .insert(
                On::<Pointer<Over>>::target_component_mut::<BackgroundColor>(move |_, bg_color| {
                    bg_color.0 = ui_color_focus;
                }),
            )
            .insert(On::<Pointer<Click>>::run(content.1));

        builder
            .style()
            .align_content(AlignContent::Center)
            .justify_content(JustifyContent::Center)
            .padding(UiRect::vertical(Val::Px(4.0)))
            .border(UiRect::all(Val::Px(1.0)))
            .border_color(text_color)
            .background_color(ui_color_none);

        builder
    }

    fn large_text_button<T>(
        &mut self,
        cfg: &Configs,
        content: (&str, impl IntoSystem<(), (), T>),
    ) -> UiBuilder<'_, Entity> {
        self.text_button(cfg, 2.0, content)
    }

    fn medium_text_button<T>(
        &mut self,
        cfg: &Configs,
        content: (&str, impl IntoSystem<(), (), T>),
    ) -> UiBuilder<'_, Entity> {
        self.text_button(cfg, 1.5, content)
    }
}

impl TextButtonUiBuilderExt for UiBuilder<'_, UiRoot> {}
impl TextButtonUiBuilderExt for UiBuilder<'_, Entity> {}

pub trait MenuUiBuilderExt0: UiColumnExt + UiRowExt + UiContainerExt {
    fn sub_menu_container<Marker>(
        &mut self,
        cfg: &Configs,
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
                .border_color(cfg.text.color)
                .background_color(cfg.menu.color_none)
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0));

            column
                .row(|row| {
                    row.medium_text_button(
                        cfg,
                        (
                            "Back",
                            |mut state: ResMut<NextState<MenuState>>,
                             mut stack: ResMut<PrevPageStack>| {
                                if let Some(prev_page) = stack.0.pop() {
                                    state.set(prev_page)
                                }
                            },
                        ),
                    );

                    row.medium_text_button(cfg, (confirm_text, confirm_action));
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
