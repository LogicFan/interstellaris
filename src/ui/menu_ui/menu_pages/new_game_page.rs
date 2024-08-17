use super::{default_button_back_action, MenuState, UiCamera, UiConfigs, UiMenuPageExt};
use crate::{
    game_map::galaxy::{gen::GalaxyGenParams, Galaxy, PrimaryGalaxy},
    states::LoadSource,
    ui::menu_ui::AppState,
};
use bevy::prelude::*;
use sickle_ui::prelude::*;

pub fn setup(
    mut commands: Commands,
    q_camera: Query<Entity, With<UiCamera>>,
    ui_config: Res<UiConfigs>,
) {
    let camera = q_camera.single();

    commands
        .ui_builder(UiRoot)
        .menu_page(
            &ui_config,
            ("Return", default_button_back_action),
            ("Start", button_next_action),
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
        .insert(Name::new("New Game Menu"))
        .insert(StateScoped(MenuState::NewGamePage));
}

fn button_next_action(mut commands: Commands, mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Loading(LoadSource::Generation));
    // entity for galaxy generation
    commands.spawn((Galaxy, PrimaryGalaxy, GalaxyGenParams::default()));
}
