use crate::traits::*;
use crate::entities::VerifiedEntity;

pub use self::indexed_vec::IndexedVec;
pub use self::entity_set::EntitySet;
pub use self::entity_map::EntityMap;

mod indexed_vec;
mod entity_set;
mod entity_map;

pub use self::component::Component;
pub use self::component_map::ComponentMap;

mod component;
mod component_map;