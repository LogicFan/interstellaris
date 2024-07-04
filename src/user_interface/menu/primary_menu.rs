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

/// Spawn the main menu.
/// # Schedule
/// `OnEnter(MenuState::MainMenu)`
pub fn spawn_main_menu(mut commands: Commands, q_camera: Query<Entity, With<PrimaryCamera>>) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .column(|column| {
            let text_style = TextStyle {
                font_size: 32.0,
                color: TEXT_COLOR,
                ..default()
            };
            column
                .text_button("New Game", text_style.clone())
                .insert(MainMenuButton::NewGame);
            column
                .text_button("Load Game", text_style.clone())
                .insert(MainMenuButton::LoadGame);
            column
                .text_button("Settings", text_style.clone())
                .insert(MainMenuButton::Settings);
            column
                .text_button("Online", text_style.clone())
                .insert(MainMenuButton::Online);
            column
                .text_button("Exit", text_style.clone())
                .insert(MainMenuButton::Exit);
        })
        .insert(TargetCamera(camera))
        .insert(PrimaryMenu)
        .insert(Name::new("Main Menu"))
        .style()
        .row_gap(Val::Px(16.0))
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

trait UiTextButtonExt {
    fn text_button(&mut self, text: &str, text_style: TextStyle) -> UiBuilder<'_, Entity>;
}

impl UiTextButtonExt for UiBuilder<'_, Entity> {
    fn text_button<'b>(&mut self, text: &str, text_style: TextStyle) -> UiBuilder<'_, Entity> {
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
            .border_color(Color::BLACK)
            .background_color(BUTTON_NONE_COLOR);

        builder
    }
}
