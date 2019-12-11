use super::*;
use rustc_hash::FxHashMap;
use crate::entities::Allocator;

#[derive(Debug, Clone)]
pub struct EntityMap<ID: Id, T> { pub values: FxHashMap<ID, T> }

impl<ID: Id, T> Default for EntityMap<ID, T> {
    fn default() -> Self {
        EntityMap { values: Default::default() }
    }
}

impl<ID: Id, T> EntityMap<ID, T> {
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

    pub fn retain(&mut self, allocator: &Allocator<ID>) {
        self.values.retain(|id, _| allocator.is_alive(*id))
    }

    pub fn retain_verified<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> impl Iterator<Item=(VerifiedEntity<'a, ID>, &T)> {
        self.retain(allocator);
        self.values
            .iter()
            .map(|(id, t)| {
                (VerifiedEntity::assert_valid(*id), t)
            })
    }

    pub fn verified<'a>(&'a self, allocator: &'a Allocator<ID>) -> impl Iterator<Item=(VerifiedEntity<'a, ID>, &T)> {
        self.values
            .iter()
            .filter_map(move |(id, t)| {
                let id = allocator.verify(*id)?;
                Some((id, t))
            })
    }
}

impl<A: Id, B: Id> EntityMap<A, B> {
    pub fn retain_verified_both<'a>(
        &'a mut self,
        allocator_a: &'a Allocator<A>,
        allocator_b: &'a Allocator<B>,
    ) -> impl Iterator<Item=(VerifiedEntity<'a, A>, VerifiedEntity<'a, B>)> {
        self.retain_both(allocator_a, allocator_b);
        self.values
            .iter()
            .map(|(a, b)| {
                (VerifiedEntity::assert_valid(*a), VerifiedEntity::assert_valid(*b))
            })
    }

    pub fn verified_both<'a>(
        &'a self,
        allocator_a: &'a Allocator<A>,
        allocator_b: &'a Allocator<B>,
    ) -> impl Iterator<Item=(VerifiedEntity<'a, A>, VerifiedEntity<'a, B>)> {
        self.values
            .iter()
            .filter_map(move |(a, b)| {
                let a = allocator_a.verify(*a)?;
                let b = allocator_b.verify(*b)?;
                Some((a, b))
            })
    }

    pub fn retain_both(&mut self, allocator_a: &Allocator<A>, allocator_b: &Allocator<B>) {
        self.values.retain(|a, b| allocator_a.is_alive(*a) && allocator_b.is_alive(*b));
    }
}

impl<ID: Id, T> Get<ID, T> for EntityMap<ID, T> {
    fn get(&self, id: &VerifiedEntity<ID>) -> Option<&T> {
        self.values.get(&id.entity)
    }

    fn get_mut(&mut self, id: &VerifiedEntity<ID>) -> Option<&mut T> {
        self.values.get_mut(&id.entity)
    }
}