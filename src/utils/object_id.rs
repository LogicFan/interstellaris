use bevy::prelude::{Component, Entity};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Component)]
pub struct ObjectId(pub Uuid);

impl Default for ObjectId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectRef {
    pub id: ObjectId,
    pub entity: Entity,
}
