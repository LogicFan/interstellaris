use super::{menu_page::MenuUiBuilderExt0, MenuState, UiCamera, UiConfigs};
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn spawn_load_game_menu(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    cfg: Res<UiConfigs>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .sub_menu_container(
            &cfg,
            "Start",
            || {},
            |parent| {
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
            },
        )
        .insert(TargetCamera(camera))
        .insert(Name::new("Load Game Menu"))
        .insert(StateScoped(MenuState::LoadGamePage));
}
