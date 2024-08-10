use super::{ui_builder_ext::MenuUiBuilderExt0, MenuState, UiCamera, UiConfigs};
use bevy::prelude::*;
use sickle_ui::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug)]
pub struct OnlineMenu;

pub fn spawn_online_menu(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    ui_settings: Res<UiConfigs>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .sub_menu_container(
            &ui_settings,
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
        .insert(OnlineMenu)
        .insert(Name::new("Online Game Menu"))
        .insert(StateScoped(MenuState::OnlineGamePage));
}
