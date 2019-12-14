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

pub trait Remove<ID: IdType, T> {
    fn remove(&mut self, id: &VerifiedEntity<ID>);
}

pub trait RemoveFrom<ID: IdType, T> {
    fn remove_from(&mut self, id: &VerifiedEntity<ID>, value: T) -> Option<T>;
}

pub trait Link<A: IdType, B: IdType> {
    fn link(&mut self, a: &VerifiedEntity<A>, b: &VerifiedEntity<B>);
}

//impl<A: IdType, B: IdType, L: Link<B, A>> Link<A, B> for L {}

pub trait Create<'a, ID: IdType, T> : Insert<ID, T> {
    fn create(&mut self, row: T, allocator: &'a mut Allocator<ID>) -> VerifiedEntity<'a, ID> {
        let id = allocator.create_entity();
        self.insert(&id, row);
        id
    }
}

pub trait CreateAndLink<'a, A: IdType, B: IdType, T> : Create<'a, B, T> + Link<A, B> {
    fn create_and_link(&mut self, owner: &VerifiedEntity<A>, row: T, allocator: &'a mut Allocator<B>) -> VerifiedEntity<'a, B> {
        let id = self.create(row, allocator);
        self.link(owner, &id);
        id
    }
}

impl<'a, A: IdType, B: IdType, T, S: Create<'a, B, T> + Link<A, B>> CreateAndLink<'a, A, B, T> for S {}

pub trait Lookup<'a, A: IdType, B: IdType> : Get<A, B> {
    fn lookup(&self, a: A, alloc_a: &Allocator<A>, alloc_b: &'a Allocator<B>) -> Option<VerifiedEntity<'a, B>> {
        alloc_a.verify(a).and_then(|a| self.lookup_verified(&a, alloc_b))
    }

    fn lookup_verified(&self, a: &VerifiedEntity<A>, alloc_b: &'a Allocator<B>) -> Option<VerifiedEntity<'a, B>> {
        self.get(a)
            .and_then(|b| alloc_b.verify(*b))
    }
}

pub trait Lookup2<'a, 'b, A: IdType, B: IdType, C: IdType> : Lookup<'a, A, B> + Lookup<'b, B, C> {
    fn lookup2(
        &self,
        a: A,
        alloc_a: &Allocator<A>,
        alloc_b: &'a Allocator<B>,
        alloc_c: &'b Allocator<C>,
    ) -> Option<VerifiedEntity<'b, C>> {
        self.lookup(a, alloc_a, alloc_b)
            .and_then(|b| self.lookup_verified(&b, alloc_c))
    }
}

impl < 'a, 'b, A: IdType, B: IdType, C: IdType, STATE: Lookup<'a, A, B> + Lookup<'b, B, C>> Lookup2<'a, 'b, A, B, C> for STATE {}

/// Impl on the type that contains both state and entities
pub trait Construct<ID: IdType, T> {
    fn construct(&mut self, value: T) -> ID;
}

pub trait Delete<OWNER: IdType, OWNED: IdType> : Get<OWNER, OWNED> + Remove<OWNER, OWNED> {
    fn delete(&mut self, id: &VerifiedEntity<OWNER>, allocator: &mut Allocator<OWNED>) {
        if let Some(owned) = self.get(id) {
            allocator.kill(*owned);
            self.remove(id);
        }
    }
}

pub trait Deconstruct<ID: IdType> {
    fn deconstruct(&mut self, id: ID);
}

pub trait Update<T> {
    fn update(state: &mut T);
}

pub trait  Split<E, S> {
    fn split(&mut self) -> (&mut E, &mut S);
}