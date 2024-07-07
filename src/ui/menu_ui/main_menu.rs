use super::ui_builder_ext::*;
use super::MenuState;
use super::UiCamera;
use super::UiSettings;
use bevy::prelude::*;
use sickle_ui::prelude::{generated::*, UiBuilderExt, UiColumnExt, UiRoot};

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MainMenu;

/// Spawn the main menu.
/// # Schedule
/// `OnEnter(MenuState::MainMenu)`
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
                .insert(MainMenuButton::NewGame);
            column
                .large_text_button(&ui_settings, "Load Game")
                .insert(MainMenuButton::LoadGame);
            column
                .large_text_button(&ui_settings, "Online")
                .insert(MainMenuButton::Online);
            column
                .large_text_button(&ui_settings, "Settings")
                .insert(MainMenuButton::Settings);
            column
                .large_text_button(&ui_settings, "Exit")
                .insert(MainMenuButton::Exit);
        })
        .insert(TargetCamera(camera))
        .insert(MainMenu)
        .insert(Name::new("Main Menu"))
        .insert(StateScoped(MenuState::MainMenu))
        .style()
        .row_gap(Val::Px(16.0))
        .align_self(AlignSelf::Center)
        .justify_self(JustifySelf::Center)
        .align_content(AlignContent::Center)
        .justify_content(JustifyContent::Center);
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MainMenuButton {
    NewGame,
    LoadGame,
    Settings,
    Online,
    Exit,
}

impl MainMenuButton {
    fn menu_state(&self) -> MenuState {
        match self {
            MainMenuButton::NewGame => MenuState::NewGame,
            MainMenuButton::LoadGame => MenuState::LoadGame,
            MainMenuButton::Settings => MenuState::SettingsMenu,
            MainMenuButton::Online => MenuState::OnlineGame,
            MainMenuButton::Exit => std::process::exit(0),
        }
    }
}

pub fn main_menu_button_handler(
    mut q_button: Query<
        (&mut BackgroundColor, &Interaction, &MainMenuButton),
        Changed<Interaction>,
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    ui_settings: Res<UiSettings>,
) {
    for (mut background_color, interaction, button) in q_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                menu_state.set(button.menu_state());
            }
            _ => (),
        }
    }
}
