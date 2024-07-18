use super::ui_builder_ext::*;
use super::MenuState;
use super::PrevPageStack;
use super::UiCamera;
use super::UiSettings;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use sickle_ui::prelude::{generated::*, UiBuilderExt, UiColumnExt, UiRoot};

/// Spawn the UI for main menu.
///
/// # Schedule
/// Enter [super::MenuState::MainMenu]
pub fn spawn_main_menu(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    ui_settings: Res<UiSettings>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column
                .large_text_button(&ui_settings, "New Game")
                .insert(On::<Pointer<Click>>::run(
                    |mut state: ResMut<NextState<MenuState>>, mut stack: ResMut<PrevPageStack>| {
                        state.set(MenuState::NewGamePage);
                        stack.0.push(MenuState::MainPage);
                    },
                ));
            column
                .large_text_button(&ui_settings, "Load Game")
                .insert(On::<Pointer<Click>>::run(
                    |mut state: ResMut<NextState<MenuState>>, mut stack: ResMut<PrevPageStack>| {
                        state.set(MenuState::LoadGamePage);
                        stack.0.push(MenuState::MainPage);
                    },
                ));
            column
                .large_text_button(&ui_settings, "Online Game")
                .insert(On::<Pointer<Click>>::run(
                    |mut state: ResMut<NextState<MenuState>>, mut stack: ResMut<PrevPageStack>| {
                        state.set(MenuState::OnlineGamePage);
                        stack.0.push(MenuState::MainPage);
                    },
                ));
            column
                .large_text_button(&ui_settings, "Settings")
                .insert(On::<Pointer<Click>>::run(
                    |mut state: ResMut<NextState<MenuState>>, mut stack: ResMut<PrevPageStack>| {
                        state.set(MenuState::SettingsPage);
                        stack.0.push(MenuState::MainPage);
                    },
                ));
            column
                .large_text_button(&ui_settings, "Exit")
                .insert(On::<Pointer<Click>>::run(|mut e: EventWriter<AppExit>| {
                    e.send(AppExit::Success);
                }));
        })
        .insert(TargetCamera(camera))
        .insert(Name::new("Main Menu"))
        .insert(StateScoped(MenuState::MainPage))
        .style()
        .row_gap(Val::Px(16.0))
        .align_self(AlignSelf::Center)
        .justify_self(JustifySelf::Center)
        .align_content(AlignContent::Center)
        .justify_content(JustifyContent::Center);
}
