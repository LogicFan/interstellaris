mod load_game_menu;
mod new_game_menu;
mod online_menu;
mod settings_menu;

pub use load_game_menu::*;
pub use new_game_menu::*;
pub use online_menu::*;
pub use settings_menu::*;

use bevy::prelude::*;
use sickle_ui::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BUTTON_NONE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.8);

trait UiSecondaryMenuExt {
    fn secondary_menu(
        &mut self,
        confirm: &str,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<'_, Entity>;
}

impl UiSecondaryMenuExt for UiBuilder<'_, UiRoot> {
    fn secondary_menu(
        &mut self,
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
                .border_color(Color::WHITE)
                .background_color(BUTTON_NONE_COLOR)
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0));

            column
                .row(|row| {
                    let text_style = TextStyle {
                        font_size: 24.0,
                        color: TEXT_COLOR,
                        ..default()
                    };

                    row.container(ButtonBundle::default(), |parent| {
                        parent.spawn(
                            TextBundle::from_section("BACK", text_style.clone())
                                .with_text_justify(JustifyText::Center),
                        );
                    })
                    .style()
                    .align_content(AlignContent::Center)
                    .justify_content(JustifyContent::Center)
                    .width(Val::Px(150.0))
                    .padding(UiRect::vertical(Val::Px(4.0)))
                    .border(UiRect::all(Val::Px(1.0)))
                    .border_color(Color::WHITE)
                    .background_color(BUTTON_NONE_COLOR);

                    row.container(ButtonBundle::default(), |parent| {
                        parent.spawn(
                            TextBundle::from_section(confirm, text_style.clone())
                                .with_text_justify(JustifyText::Center),
                        );
                    })
                    .style()
                    .align_content(AlignContent::Center)
                    .justify_content(JustifyContent::Center)
                    .width(Val::Px(150.0))
                    .padding(UiRect::vertical(Val::Px(4.0)))
                    .border(UiRect::all(Val::Px(1.0)))
                    .border_color(Color::WHITE)
                    .background_color(BUTTON_NONE_COLOR);
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
