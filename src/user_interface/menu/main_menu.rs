use crate::user_interface::PrimaryCamera;
use bevy::prelude::*;
use sickle_ui::prelude::*;

use super::MenuState;

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MainMenuItem;

/// helper function for creating buttons for main menu
fn spawn_button<T>(builder: &mut UiBuilder<Entity>, text: &str, marker: T)
where
    T: Component,
{
    let text_style = TextStyle {
        font_size: 32.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };

    builder
        .container(ButtonBundle::default(), |children| {
            children
                .spawn(
                    TextBundle::from_section(text, text_style)
                        .with_text_justify(JustifyText::Center),
                )
                .insert(MainMenuItem);
        })
        .insert(MainMenuItem)
        .insert(marker)
        .style()
        .align_content(AlignContent::Center)
        .justify_content(JustifyContent::Center)
        .width(Val::Px(200.0))
        .padding(UiRect::vertical(Val::Px(4.0)))
        .margin(UiRect::vertical(Val::Px(8.0)))
        .border(UiRect::all(Val::Px(1.0)))
        .border_color(Color::BLACK)
        .background_color(Color::rgba(0.0, 0.0, 0.0, 0.5));
}

/// Spawn the main menu.
/// # Schedule
/// `OnEnter(MenuState::MainMenu)`
pub fn spawn_main_menu(mut commands: Commands, q_camera: Query<Entity, With<PrimaryCamera>>) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .column(|column| {
            spawn_button(column, "New Game", NewGameButton);
            spawn_button(column, "Load Game", LoadGameButton);
            spawn_button(column, "Settings", SettingsButton);
            spawn_button(column, "Online", OnlineButton);
            spawn_button(column, "Exit", ExitButton);
        })
        .insert(TargetCamera(camera))
        .insert(MainMenuItem)
        .insert(Name::new("Main Menu"))
        .style()
        .align_self(AlignSelf::Center)
        .justify_self(JustifySelf::Center)
        .align_content(AlignContent::Center)
        .justify_content(JustifyContent::Center);
}

/// Despawn the background for Menu.
/// # Schedule
/// `OnExit(MenuState::MainMenu)`
pub fn despawn_main_menu(mut commands: Commands, q_background: Query<Entity, With<MainMenuItem>>) {
    for entity in q_background.iter() {
        commands.entity(entity).despawn();
    }
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NewGameButton;

pub fn new_game_handler(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NewGameButton>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if let Some(button) = q_button.get_single_mut().ok() {
        let (interaction, mut background_color) = button;
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
                menu_state.set(MenuState::NewGame);
            }
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            }
        }
    }
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LoadGameButton;

pub fn load_game_handler(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<LoadGameButton>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if let Some(button) = q_button.get_single_mut().ok() {
        let (interaction, mut background_color) = button;
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
                menu_state.set(MenuState::LoadGame);
            }
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            }
        }
    }
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SettingsButton;

pub fn settings_handler(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if let Some(button) = q_button.get_single_mut().ok() {
        let (interaction, mut background_color) = button;
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
                menu_state.set(MenuState::Settings);
            }
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            }
        }
    }
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct OnlineButton;

pub fn online_handler(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<OnlineButton>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if let Some(button) = q_button.get_single_mut().ok() {
        let (interaction, mut background_color) = button;
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
                menu_state.set(MenuState::Online);
            }
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            }
        }
    }
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ExitButton;

pub fn exit_handler(
    mut q_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
) {
    if let Some(button) = q_button.get_single_mut().ok() {
        let (interaction, mut background_color) = button;
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
                std::process::exit(0);
            }
            Interaction::Hovered => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.5);
            }
        }
    }
}
