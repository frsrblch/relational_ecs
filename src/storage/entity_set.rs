use super::*;
use rustc_hash::FxHashSet;
use crate::entities::Allocator;

#[derive(Debug, Clone)]
pub struct EntitySet<ID: IdType> { pub values: FxHashSet<ID> }

impl<ID: IdType> Default for EntitySet<ID> {
    fn default() -> Self {
        EntitySet { values: Default::default() }
    }
}

impl<ID: IdType> EntitySet<ID> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<ID> {
        self.values.iter()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn insert(&mut self, value: ID) {
        self.values.insert(value);
    }

    pub fn remove(&mut self, value: &ID) -> Option<ID> {
        if self.values.remove(value) {
            Some(*value)
        } else {
            None
        }
    }

    pub fn contains(&self, value: &ID) -> bool {
        self.values.contains(value)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn retain(&mut self, allocator: &Allocator<ID>) {
        self.values.retain(|id| allocator.is_alive(*id));
    }

    pub fn retain_verified<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> impl Iterator<Item=VerifiedEntity<'a, ID>> {
        self.retain(allocator);
        self.values
            .iter()
            .map(|id| VerifiedEntity::assert_valid(*id))
    }

    pub fn verified<'a>(&'a self, allocator: &'a Allocator<ID>) -> impl Iterator<Item=VerifiedEntity<'a, ID>> {
        self.values
            .iter()
            .filter_map(move |id| allocator.verify(*id))
    }
}
