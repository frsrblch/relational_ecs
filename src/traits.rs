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

pub trait Insert<ID: IdType, T> {
    fn insert(&mut self, id: &VerifiedEntity<ID>, value: T);
}