use bevy::ecs::system::SystemId;
use bevy::prelude::Component;
use bevy::utils::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum OneShotSystem {
    UiEnableVirtualPointer,
    UiDisableVirtualPointer,
}

#[derive(Debug, Clone, Default, Component)]
pub struct OneShotSystemMap(pub HashMap<OneShotSystem, SystemId>);
