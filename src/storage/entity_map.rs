use super::*;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct EntityMap<ID: IdType, T> { pub values: FxHashMap<ID, T> }

impl<ID: IdType, T> Default for EntityMap<ID, T> {
    fn default() -> Self {
        EntityMap { values: Default::default() }
    }
}

impl<ID: IdType, T> EntityMap<ID, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<ID, T> {
        self.values.iter()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn insert(&mut self, id: &VerifiedEntity<ID>, value: T) {
        self.values.insert(id.entity, value);
    }

    pub fn remove(&mut self, id: &VerifiedEntity<ID>) -> Option<T> {
        self.values.remove(&id.entity)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<ID: IdType, T> Get<ID, T> for EntityMap<ID, T> {
    fn get(&self, id: &VerifiedEntity<ID>) -> Option<&T> {
        self.values.get(&id.entity)
    }

    fn get_mut(&mut self, id: &VerifiedEntity<ID>) -> Option<&mut T> {
        self.values.get_mut(&id.entity)
    }
}