//! Main page of the menu.
use super::MenuState;
use super::UiCamera;
use super::UiConfigs;
use bevy::prelude::*;
use bevy_mod_picking::events::*;
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::prelude::Pickable;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiContainerExt;
use sickle_ui::prelude::{generated::*, UiBuilderExt, UiColumnExt, UiRoot};

/// Spawn the UI for main menu.
///
/// # Schedule
/// Enter [super::MenuState::MainPage]
pub fn setup(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    ui_config: Res<UiConfigs>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .main_page(&ui_config)
        .insert(TargetCamera(camera))
        .insert(Name::new("Main Page"))
        .insert(StateScoped(MenuState::MainPage));
}

/// The UI builder for main page.
trait UiMainPageExt: UiContainerExt + UiColumnExt {
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
                .spawn(TextBundle::from_section(content.0, text_style))
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
            .padding(UiRect::all(Val::Px(4.0)))
            .border(UiRect::all(Val::Px(1.0)))
            .border_color(text_color)
            .background_color(ui_color_none);

        builder
    }

    fn main_page(&mut self, cfg: &UiConfigs) -> UiBuilder<'_, Entity> {
        let mut builder = self.column(|column| {
            column
                ._button(
                    &cfg,
                    ("New Game", |mut state: ResMut<NextState<MenuState>>| {
                        state.set(MenuState::NewGamePage);
                    }),
                )
                .style()
                .width(Val::Percent(100.0));
            column
                ._button(
                    &cfg,
                    ("Load Game", |mut state: ResMut<NextState<MenuState>>| {
                        state.set(MenuState::LoadGamePage);
                    }),
                )
                .style()
                .width(Val::Percent(100.0));
            column
                ._button(
                    &cfg,
                    ("Online Game", |mut state: ResMut<NextState<MenuState>>| {
                        state.set(MenuState::OnlineGamePage);
                    }),
                )
                .style()
                .width(Val::Percent(100.0));
            column
                ._button(
                    &cfg,
                    ("Settings", |mut state: ResMut<NextState<MenuState>>| {
                        state.set(MenuState::SettingsPage);
                    }),
                )
                .style()
                .width(Val::Percent(100.0));
            column
                ._button(
                    &cfg,
                    ("Exit", |mut app_exit: EventWriter<AppExit>| {
                        app_exit.send(AppExit::Success);
                    }),
                )
                .style()
                .width(Val::Percent(100.0));
        });

        builder.insert(Pickable::IGNORE);

        builder
            .style()
            .width(Val::Px(200.0 * cfg.scale))
            .row_gap(Val::Px(32.0))
            .align_self(AlignSelf::Center)
            .left(Val::Percent(10.0))
            .align_content(AlignContent::Center)
            .justify_content(JustifyContent::Center);

        builder
    }
}

impl UiMainPageExt for UiBuilder<'_, UiRoot> {}
impl UiMainPageExt for UiBuilder<'_, Entity> {}
