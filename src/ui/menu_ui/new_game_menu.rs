use super::{ui_builder_ext::MenuUiBuilderExt0, MenuState, UiCamera, UiSettings};
use crate::{game_map::GameMapGenArgs, ui::menu_ui::AppState};
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn spawn_new_game_menu(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    ui_settings: Res<UiSettings>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .sub_menu_container(&ui_settings, "Start", confirm_button_handler, |parent| {
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
        .insert(Name::new("New Game Menu"))
        .insert(StateScoped(MenuState::NewGame));
}

fn confirm_button_handler(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Loading);
    commands.spawn(GameMapGenArgs::default());
}