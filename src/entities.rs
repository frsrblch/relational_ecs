use std::marker::PhantomData;
use std::num::NonZeroU32;
use crate::traits::IdType;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Generation(NonZeroU32);

impl Generation {
    pub fn next(self) -> Self {
        let next_gen = NonZeroU32::new(self.0.get() + 1).unwrap();
        Generation(next_gen)
    }

    pub fn value(self) -> u32 {
        self.0.get()
    }
}

impl Default for Generation {
    fn default() -> Self {
        Generation(NonZeroU32::new(1).unwrap())
    }
}

#[derive(Debug)]
pub struct Allocator<ID: IdType> {
    generations: Vec<Generation>,
    dead: Vec<usize>,
    living: Vec<Option<ID>>,
}

impl<ID: IdType> Default for Allocator<ID> {
    fn default() -> Self {
        Self {
            generations: vec![],
            dead: vec![],
            living: vec![],
        }
    }
}

impl<ID: IdType> Allocator<ID> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_entity(&mut self) -> VerifiedEntity<ID> {
        if let Some(index) = self.dead.pop() {
            if let Some(gen) = self.generations.get(index) {
                let entity = ID::create(index, *gen);
                self.living[index] = Some(entity);
                return VerifiedEntity::assert_valid(entity)
            }
        }

        let index = self.get_new_index();
        let entity = ID::new(index as u32);
        self.living.push(Some(entity));
        self.generations.push(entity.generation());

        VerifiedEntity::assert_valid(entity)
    }

    fn get_new_index(&mut self) -> usize {
        self.generations.len()
    }

    pub fn ids(&self) -> impl Iterator<Item = VerifiedEntity<ID>> {
        self.living.iter()
            .filter_map(|id| {
                id.map(|i| VerifiedEntity::assert_valid(i))
            })
    }

    pub fn kill(&mut self, id: ID) -> Option<()> {
        if self.is_alive(id) {
            let gen = &mut self.generations[id.index()];
            *gen = gen.next();
            self.dead.push(id.index());
            self.living[id.index()] = None;
            return Some(());
        }

        None
    }

    pub fn is_alive(&self, entity: ID) -> bool {
        if let Some(gen) = self.generations.get(entity.index()) {
            entity.generation() == *gen
        } else {
            false
        }
    }

    pub fn verify(&self, entity: ID) -> Option<VerifiedEntity<ID>> {
        if self.is_alive(entity) {
            Some(VerifiedEntity::assert_valid(entity))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct VerifiedEntity<'a,ID: IdType> {
    pub entity: ID,
    marker: PhantomData<&'a Allocator<ID>>,
}

impl<'a, ID: IdType> VerifiedEntity<'a, ID> {
    pub fn assert_valid(entity: ID) -> Self {
        VerifiedEntity { entity, marker: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    id_type!(TestId);

    fn assert_id(id: TestId, index: u32, generation: u32) {
        assert_eq!(index, id.0);
        assert_eq!(generation, (id.1).0.get());
    }

    #[test]
    fn create_entity() {
        let mut allocator = Allocator::<TestId>::new();
        let id = allocator.create_entity().entity;

        assert_id(id, 0, 1);
        assert!(allocator.is_alive(id));
    }

    #[test]
    fn kill_entity() {
        let mut allocator = Allocator::<TestId>::new();
        let id = allocator.create_entity().entity;

        allocator.kill(id);

        assert!(!allocator.is_alive(id));
    }

    #[test]
    fn reuses_dead() {
        let mut allocator = Allocator::<TestId>::new();
        let id = allocator.create_entity().entity;
        allocator.kill(id);

        let id = allocator.create_entity().entity;

        assert!(allocator.is_alive(id));
        assert_id(id, 0, 2);
    }

    #[test]
    fn create_second() {
        let mut allocator = Allocator::<TestId>::new();
        let _ = allocator.create_entity();
        let id = allocator.create_entity().entity;

        assert_id(id, 1, 1);
        assert!(allocator.is_alive(id));
    }

    #[test]
    fn dead_id_is_not_alive1() {
        let mut allocator = Allocator::<TestId>::new();
        let dead = allocator.create_entity().entity;
        allocator.kill(dead);

        assert!(!allocator.is_alive(dead));
    }

    #[test]
    fn dead_id_is_not_alive2() {
        let mut allocator = Allocator::<TestId>::new();
        let dead = allocator.create_entity().entity;
        allocator.kill(dead);
        let live = allocator.create_entity().entity;

        assert_eq!(dead.0, live.0); // same index
        assert!(!allocator.is_alive(dead));
        assert!(allocator.is_alive(live));
    }

    #[test]
    fn verify_returns_some_if_alive() {
        let mut allocator = Allocator::<TestId>::new();
        let id = allocator.create_entity().entity;

        assert!(allocator.verify(id).is_some());
    }

    #[test]
    fn verify_returns_none_if_dead() {
        let mut allocator = Allocator::<TestId>::new();
        let id = allocator.create_entity().entity;
        allocator.kill(id);

        assert!(allocator.verify(id).is_none());
    }

    #[test]
    fn ids_returns_iterator_of_the_living() {
        let mut allocator = Allocator::<TestId>::new();

        let id0 = allocator.create_entity().entity;
        let id1 = allocator.create_entity().entity;
        let id2 = allocator.create_entity().entity;

        allocator.kill(id1);

        let ids = &mut allocator.ids();

        assert_eq!(id0, ids.next().unwrap().entity);
        assert_eq!(id2, ids.next().unwrap().entity);
        assert!(ids.next().is_none());
    }

//    /// THIS TEST PASSES IF IT DOES NOT COMPILE
//    #[test]
//    fn allocator_verify_lifetime_test() {
//        let mut allocator = Allocator::<TestId>::new();
//        let id = allocator.create_entity().entity;
//
//        let vid = allocator.verify(id).unwrap();
//
//        drop(allocator);
//
//        dbg!(vid);
//    }
//
//    /// THIS TEST PASSES IF IT DOES NOT COMPILE
//    #[test]
//    fn allocator_create_lifetime_test() {
//        let mut allocator = Allocator::<TestId>::new();
//        let vid = allocator.create_entity();
//
//        drop(allocator);
//
//        dbg!(vid);
//    }
}