mod new_game_page;

pub use super::{UiConfigs, UiCamera, MenuState};
use bevy::prelude::*;
use bevy_mod_picking::{
    events::*,
    prelude::{On, Pickable},
};
use sickle_ui::prelude::{generated::*, UiBuilder, UiColumnExt, UiContainerExt, UiRoot, UiRowExt};
pub use new_game_page::setup as setup_new_game_page;

fn default_button_back_action(mut state: ResMut<NextState<MenuState>>) {
    state.set(MenuState::MainPage)
}

pub trait UiMenuPageExt: UiContainerExt + UiColumnExt {
    fn _button<T>(
        &mut self,
        cfg: &UiConfigs,
        content: (&str, impl IntoSystem<(), (), T>),
    ) -> UiBuilder<'_, Entity> {
        let scale = cfg.scale * 1.5;
        let text_color = cfg.text.color;

        let text_style = TextStyle {
            font: cfg.text.font.clone(),
            font_size: 16.0 * scale,
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
            .width(Val::Px(100.0))
            .padding(UiRect::all(Val::Px(4.0)))
            .border(UiRect::all(Val::Px(1.0)))
            .border_color(text_color)
            .background_color(ui_color_none);

        builder
    }

    fn menu_page<S, T>(
        &mut self,
        cfg: &UiConfigs,
        back: (&str, impl IntoSystem<(), (), S>),
        next: (&str, impl IntoSystem<(), (), T>),
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
                    row._button(cfg, back);
                    row._button(cfg, next);
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
}

impl UiMenuPageExt for UiBuilder<'_, UiRoot> {}
impl UiMenuPageExt for UiBuilder<'_, Entity> {}
