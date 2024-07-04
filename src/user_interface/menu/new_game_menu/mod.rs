// use crate::user_interface::PrimaryCamera;
// use bevy::prelude::*;
// use sickle_ui::prelude::*;

// /// A marker component for all main menu items.
// #[derive(Component, Clone, Copy, Debug, Default)]
// pub struct NewGameMenuItem;

// /// Spawn the main menu.
// /// # Schedule
// /// `OnEnter(MenuState::MainMenu)`
// pub fn spawn_new_game_menu(mut commands: Commands, q_camera: Query<Entity, With<PrimaryCamera>>) {
//     let camera = q_camera.single();

//     commands
//         .ui_builder(UiRoot)
//         .column(|column| {
//             column.spawn(
//                 TextBundle::from_section("abc", TextStyle::default())
//                     .with_text_justify(JustifyText::Center),
//             );
//         })
//         .insert(TargetCamera(camera))
//         .insert(NewGameMenuItem)
//         .insert(Name::new("New Game Menu"))
//         .style()
//         .background_color(Color::BLACK)
//         .menu_container_style();
// }

// /// Despawn the main menu.
// /// # Schedule
// /// `OnExit(MenuState::MainMenu)`
// pub fn despawn_new_game_menu(
//     mut commands: Commands,
//     q_new_game_menu: Query<Entity, With<NewGameMenuItem>>,
// ) {
//     let new_game_menu = q_new_game_menu.single();
//     commands.entity(new_game_menu).despawn_recursive();
// }
