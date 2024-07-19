//! Utility structs and functions that does not belongs
//! to any other module.

mod object_id;
mod random;

pub use object_id::{ObjectId, ObjectRef};
pub use random::{default_rng, RngExt};
