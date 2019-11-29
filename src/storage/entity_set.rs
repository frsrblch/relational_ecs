use super::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct EntitySet<ID: IdType>(FxHashSet<ID>);

impl<ID: IdType> Default for EntitySet<ID> {
    fn default() -> Self {
        EntitySet(Default::default())
    }
}

impl<ID: IdType> EntitySet<ID> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<ID> {
        self.0.iter()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn insert(&mut self, value: ID) {
        self.0.insert(value);
    }

    pub fn remove(&mut self, value: &ID) -> Option<ID> {
        if self.0.remove(value) {
            Some(*value)
        } else {
            None
        }
    }

    pub fn contains(&self, value: &ID) -> bool {
        self.0.contains(value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
