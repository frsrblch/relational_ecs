use std::hash::Hash;
use crate::entities::*;

pub trait IdType: Copy + Eq + Hash + Ord {
    fn new(index: u32) -> Self;
    fn create(index: usize, gen: Generation) -> Self;
    fn index(&self) -> usize;
    fn generation(&self) -> Generation;
}

pub trait Entities<ID: IdType> {
    fn verify(&self, id: ID) -> Option<VerifiedEntity<ID>>;
    fn is_alive(&self, id: ID) -> bool;
    fn create(&mut self) -> VerifiedEntity<ID>;
    fn delete(&mut self, id: ID);
}

pub trait Get<ID: IdType, T> {
    fn get(&self, id: &VerifiedEntity<ID>) -> Option<&T>;
    fn get_mut(&mut self, id: &VerifiedEntity<ID>) -> Option<&mut T>;
}

pub trait Link<A: IdType, B: IdType> : Insert<A, B> + Insert<B, A> {
    fn link(&mut self, a: &VerifiedEntity<A>, b: &VerifiedEntity<B>) {
        self.insert(a, b.entity);
        self.insert(b, a.entity);
    }
}

pub trait Remove<ID: IdType, T> {
    fn remove(&mut self, id: &VerifiedEntity<ID>, value: T) -> Option<T>;
}

pub trait Insert<ID: IdType, T> {
    fn insert(&mut self, id: &VerifiedEntity<ID>, value: T);
}

pub trait Create<'a, ID: IdType, T> : Insert<ID, T> {
    fn create(&mut self, row: T, allocator: &'a mut Allocator<ID>) -> VerifiedEntity<'a, ID> {
        let id = allocator.create_entity();
        self.insert(&id, row);
        id
    }
}
