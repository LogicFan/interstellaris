use super::MenuState;
use crate::user_interface::PrimaryCamera;
use bevy::prelude::*;
use sickle_ui::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BUTTON_NONE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.8);
const BUTTON_FOCUS_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.6);

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PrimaryMenu;

/// helper function for creating buttons for main menu
fn spawn_button(builder: &mut UiBuilder<Entity>, text: &str, button: MainMenuButton) {
    let text_style = TextStyle {
        font_size: 32.0,
        color: TEXT_COLOR,
        ..default()
    };

    builder
        .container(ButtonBundle::default(), |parent| {
            parent.spawn(
                TextBundle::from_section(text, text_style).with_text_justify(JustifyText::Center),
            );
        })
        .insert(button)
        .style()
        .align_content(AlignContent::Center)
        .justify_content(JustifyContent::Center)
        .width(Val::Px(200.0))
        .padding(UiRect::vertical(Val::Px(4.0)))
        .margin(UiRect::vertical(Val::Px(8.0)))
        .border(UiRect::all(Val::Px(1.0)))
        .border_color(Color::BLACK)
        .background_color(BUTTON_NONE_COLOR)
        .entity_commands();
}

/// Spawn the main menu.
/// # Schedule
/// `OnEnter(MenuState::MainMenu)`
pub fn spawn_main_menu(mut commands: Commands, q_camera: Query<Entity, With<PrimaryCamera>>) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .column(|column| {
            spawn_button(column, "New Game", MainMenuButton::NewGame);
            spawn_button(column, "Load Game", MainMenuButton::LoadGame);
            spawn_button(column, "Settings", MainMenuButton::Settings);
            spawn_button(column, "Online", MainMenuButton::Online);
            spawn_button(column, "Exit", MainMenuButton::Exit);
        })
        .insert(TargetCamera(camera))
        .insert(PrimaryMenu)
        .insert(Name::new("Main Menu"))
        .style()
        .align_self(AlignSelf::Center)
        .justify_self(JustifySelf::Center)
        .align_content(AlignContent::Center)
        .justify_content(JustifyContent::Center);
}

/// Despawn the main menu.
/// # Schedule
/// `OnExit(MenuState::MainMenu)`
pub fn despawn_main_menu(mut commands: Commands, q_main_menu: Query<Entity, With<PrimaryMenu>>) {
    let main_menu = q_main_menu.single();
    commands.entity(main_menu).despawn_recursive();
}

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug)]
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
            MainMenuButton::NewGame => MenuState::NewGameMenu,
            MainMenuButton::LoadGame => MenuState::LoadGameMenu,
            MainMenuButton::Settings => MenuState::SettingsMenu,
            MainMenuButton::Online => MenuState::OnlineMenu,
            MainMenuButton::Exit => std::process::exit(0),
        }
    }
}

pub fn primary_menu_button_handler(
    mut q_button: Query<
        (&mut BackgroundColor, &Interaction, &MainMenuButton),
        Changed<Interaction>,
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (mut background_color, interaction, button) in q_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = BUTTON_FOCUS_COLOR;
                menu_state.set(button.menu_state());
            }
            Interaction::Hovered => {
                background_color.0 = BUTTON_FOCUS_COLOR;
            }
            Interaction::None => {
                background_color.0 = BUTTON_NONE_COLOR;
            }
        }
    }
}
