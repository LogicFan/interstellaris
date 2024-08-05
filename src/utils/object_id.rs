//! Id for game objects.

use bevy::prelude::{Component, Entity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A stable unique id for all objects in the game
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(pub Uuid);

impl Default for ObjectId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

/// A struct to record [ObjectId] and [Entity] info,
/// usually useful or record relation between
/// game objects.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectRef {
    /// The stable id for the game object
    pub object_id: ObjectId,
    /// The id used by bevy internally. It will change
    /// at each run.
    pub entity: Entity,
}

impl ObjectRef {
    pub fn new(entity: Entity, object_id: ObjectId) -> Self {
        Self { object_id, entity }
    }
}

impl Serialize for ObjectRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.object_id.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ObjectRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        Ok(Self {
            object_id: id,
            entity: Entity::PLACEHOLDER,
        })
    }
}
