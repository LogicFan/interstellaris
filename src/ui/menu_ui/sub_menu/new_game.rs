use super::{SubMenuButton, UiCamera, UiSecondaryMenuExt, UiSettings};
use crate::{game_map::GameMapGenArgs, ui::menu_ui::AppState};
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NewGameMenu;

pub fn spawn_new_game_menu(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    ui_settings: Res<UiSettings>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .secondary_menu(&ui_settings, "Start", |parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "Under Construction",
                        TextStyle {
                            font_size: 100.0,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center),
                )
                .style()
                .align_self(AlignSelf::Center)
                .justify_self(JustifySelf::Center);
        })
        .insert(TargetCamera(camera))
        .insert(NewGameMenu)
        .insert(Name::new("New Game Menu"));
}

pub fn confirm_button_handler(
    mut commands: Commands,
    mut q_button: Query<(&Interaction, &SubMenuButton), Changed<Interaction>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in q_button.iter_mut() {
        if *interaction == Interaction::Pressed && *button == SubMenuButton::Confirm {
            app_state.set(AppState::LoadScene);
            commands.spawn(GameMapGenArgs::default());
        }
    }
}
