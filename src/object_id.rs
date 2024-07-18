use bevy::prelude::{Component, Entity};
use uuid::Uuid;

/// A unique id for all objects in the game
#[derive(Debug, Clone, Copy, Component)]
pub struct ObjectId(pub Uuid);

impl Default for ObjectId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// A struct to record [ObjectId] and [Entity] info,
/// usually useful or record relation between
/// game objects.
#[derive(Debug, Clone, Copy)]
pub struct ObjectRef {
    /// The stable id for the game object
    pub id: ObjectId,
    /// The id used by bevy internally. It will change
    /// at each run.
    pub entity: Entity,
}
