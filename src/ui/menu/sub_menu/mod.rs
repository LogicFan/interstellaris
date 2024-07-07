pub mod load_game;
pub mod new_game;
pub mod online;
pub mod settings;

pub use super::UiCamera;
pub use super::UiSettings;

use super::MenuState;
use bevy::prelude::*;
use sickle_ui::prelude::*;

trait UiSecondaryMenuExt {
    fn secondary_menu(
        &mut self,
        settings: &UiSettings,
        confirm: &str,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity>;
}

impl UiSecondaryMenuExt for UiBuilder<'_, UiRoot> {
    fn secondary_menu(
        &mut self,
        settings: &UiSettings,
        confirm: &str,
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
                .background_color(settings.background_color_1)
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0));

            column
                .row(|row| {
                    let text_style = TextStyle {
                        font: settings.font.clone(),
                        font_size: 24.0 * settings.ui_scale,
                        color: settings.text_color,
                    };

                    row.container(ButtonBundle::default(), |parent| {
                        parent.spawn(
                            TextBundle::from_section("Return", text_style.clone())
                                .with_text_justify(JustifyText::Center),
                        );
                    })
                    .insert(SubMenuButton::Return)
                    .style()
                    .align_content(AlignContent::Center)
                    .justify_content(JustifyContent::Center)
                    .width(Val::Px(150.0))
                    .padding(UiRect::vertical(Val::Px(4.0)))
                    .border(UiRect::all(Val::Px(1.0)))
                    .border_color(settings.text_color)
                    .background_color(settings.background_color_1);

                    row.container(ButtonBundle::default(), |parent| {
                        parent.spawn(
                            TextBundle::from_section(confirm, text_style.clone())
                                .with_text_justify(JustifyText::Center),
                        );
                    })
                    .insert(SubMenuButton::Confirm)
                    .style()
                    .align_content(AlignContent::Center)
                    .justify_content(JustifyContent::Center)
                    .width(Val::Px(150.0))
                    .padding(UiRect::vertical(Val::Px(4.0)))
                    .border(UiRect::all(Val::Px(1.0)))
                    .border_color(settings.text_color)
                    .background_color(settings.background_color_1);
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

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubMenuButton {
    Return,
    Confirm,
}

pub fn sub_menu_button_handler(
    mut q_button: Query<(&mut BackgroundColor, &Interaction, &SubMenuButton), Changed<Interaction>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    ui_settings: Res<UiSettings>,
) {
    for (mut background_color, interaction, button) in q_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = ui_settings.background_color_2;
                if *button == SubMenuButton::Return {
                    menu_state.set(MenuState::MainMenu);
                }
            }
            Interaction::Hovered => {
                background_color.0 = ui_settings.background_color_2;
            }
            Interaction::None => {
                background_color.0 = ui_settings.background_color_1;
            }
        }
    }
}
