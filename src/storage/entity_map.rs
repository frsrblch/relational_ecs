use super::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct EntityMap<ID: IdType, T>(FxHashMap<ID, T>);

impl<ID: IdType, T> Default for EntityMap<ID, T> {
    fn default() -> Self {
        EntityMap(Default::default())
    }
}

impl<ID: IdType, T> EntityMap<ID, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<ID, T> {
        self.0.iter()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn insert(&mut self, id: &VerifiedEntity<ID>, value: T) {
        self.0.insert(id.entity, value);
    }

    pub fn remove(&mut self, id: &VerifiedEntity<ID>) -> Option<T> {
        self.0.remove(&id.entity)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<ID: IdType, T> Get<ID, T> for EntityMap<ID, T> {
    fn get(&self, id: &VerifiedEntity<ID>) -> Option<&T> {
        self.0.get(&id.entity)
    }

    fn get_mut(&mut self, id: &VerifiedEntity<ID>) -> Option<&mut T> {
        self.0.get_mut(&id.entity)
    }
}