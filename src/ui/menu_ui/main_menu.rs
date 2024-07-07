use super::MenuState;
use super::UiCamera;
use super::UiSettings;
use bevy::prelude::*;
use sickle_ui::prelude::*;

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
                .text_button(&ui_settings, "New Game")
                .insert(MainMenuButton::NewGame);
            column
                .text_button(&ui_settings, "Load Game")
                .insert(MainMenuButton::LoadGame);
            column
                .text_button(&ui_settings, "Online")
                .insert(MainMenuButton::Online);
            column
                .text_button(&ui_settings, "Settings")
                .insert(MainMenuButton::Settings);
            column
                .text_button(&ui_settings, "Exit")
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
                background_color.0 = ui_settings.background_color_2;
                menu_state.set(button.menu_state());
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

trait UiTextButtonExt {
    fn text_button(&mut self, settings: &UiSettings, text: &str) -> UiBuilder<'_, Entity>;
}

impl UiTextButtonExt for UiBuilder<'_, Entity> {
    fn text_button<'b>(&mut self, settings: &UiSettings, text: &str) -> UiBuilder<'_, Entity> {
        let text_style = TextStyle {
            font: settings.font.clone(),
            font_size: 32.0 * settings.ui_scale,
            color: settings.text_color,
        };

        let mut builder = self.container(ButtonBundle::default(), |parent| {
            parent.spawn(
                TextBundle::from_section(text, text_style).with_text_justify(JustifyText::Center),
            );
        });

        builder
            .style()
            .align_content(AlignContent::Center)
            .justify_content(JustifyContent::Center)
            .width(Val::Px(200.0))
            .padding(UiRect::vertical(Val::Px(4.0)))
            .border(UiRect::all(Val::Px(1.0)))
            .border_color(settings.text_color)
            .background_color(settings.background_color_1);

        builder
    }
}
