use super::UiSecondaryMenuExt;
use crate::user_interface::PrimaryCamera;
use bevy::prelude::*;
use sickle_ui::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BUTTON_NONE_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.8);
const BUTTON_FOCUS_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.6);

/// A marker component for all main menu items.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NewGameMenu;

/// Spawn the main menu.
/// # Schedule
/// `OnEnter(MenuState::MainMenu)`
pub fn spawn_new_game_menu(mut commands: Commands, q_camera: Query<Entity, With<PrimaryCamera>>) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .secondary_menu("Start", |parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Under Construction",
                    TextStyle {
                        font_size: 100.0,
                        ..default()
                    },
                ).with_text_justify(JustifyText::Center))
                .style()
                .align_self(AlignSelf::Center)
                .justify_self(JustifySelf::Center);
        })
        .insert(TargetCamera(camera))
        .insert(NewGameMenu)
        .insert(Name::new("New Game Menu"));
}
