use std::marker::PhantomData;
use super::*;
use std::ops::{Index, IndexMut};
use crate::entities::Allocator;

#[derive(Debug)]
pub struct IndexedVec<ID: IdType, T> {
    pub values: Vec<T>,
    marker: PhantomData<ID>,
}

impl<ID: IdType, T> Default for IndexedVec<ID, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<ID: IdType, T> IndexedVec<ID, T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            marker: PhantomData,
        }
    }
}

impl<ID: IdType, T> Get<ID, T> for IndexedVec<ID, T> {
    fn get(&self, id: &VerifiedEntity<ID>) -> Option<&T> {
        self.values.get(id.entity.index())
    }

    fn get_mut(&mut self, id: &VerifiedEntity<ID>) -> Option<&mut T> {
        self.values.get_mut(id.entity.index())
    }
}

impl<ID: IdType, T> Insert<ID, T> for IndexedVec<ID, T> {
    fn insert(&mut self, id: &VerifiedEntity<ID>, value: T) {
        let id = id.entity;
        match self.values.len() {
            len if len > id.index() => self.values[id.index()] = value,
            len if len == id.index() => self.values.push(value),
            _ => panic!(format!("entity index out of bounds: {:?} len: {}", id, self.values.len())),
        };
    }
}

impl<'a, ID: IdType, T> Index<&'a VerifiedEntity<'a, ID>> for IndexedVec<ID, T> {
    type Output = T;

    fn index(&self, index: &'a VerifiedEntity<'a, ID>) -> &Self::Output {
        &self.values[index.entity.index()]
    }
}

impl<'a, ID: IdType, T> IndexMut<&'a VerifiedEntity<'a, ID>> for IndexedVec<ID, T> {
    fn index_mut(&mut self, index: &'a VerifiedEntity<'a, ID>) -> &mut Self::Output {
        &mut self.values[index.entity.index()]
    }
}

impl<A: IdType, B: IdType> IndexedVec<A, Option<B>> {
    pub fn retain(&mut self, allocator: &Allocator<B>) {
        self.values
            .iter_mut()
            .for_each(|b| {
                if let Some(inner) = b {
                    if !allocator.is_alive(*inner) {
                        *b = None;
                    }
                }
            })
    }
}

impl<A: IdType, B: IdType> IndexedVec<A, B> {
    pub fn verified_both<'a>(
        &'a self,
        allocator_a: &'a Allocator<A>,
        allocator_b: &'a Allocator<B>,
    ) -> impl Iterator<Item=(VerifiedEntity<A>, VerifiedEntity<B>)> {
        allocator_a
            .ids()
            .filter_map(move |a| {
                let b = self[&a];
                let b = allocator_b.verify(b)?;
                Some((a, b))
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::Allocator;
    use super::*;

    id_type!(TestId);

    #[test]
    fn insert_and_get() {
        let mut allocator = Allocator::<TestId>::new();
        let mut storage = IndexedVec::<TestId, u32>::new();

        let id = allocator.create_entity();

        storage.insert(&id, 5);

        assert_eq!(Some(&5), storage.get(&id));
    }

    #[test]
    #[should_panic]
    fn insert_invalid_entity_panics() {
        let mut allocator = Allocator::<TestId>::new();
        let mut storage = IndexedVec::<TestId, u32>::new();

        let _id = allocator.create_entity();
        let id = allocator.create_entity();

        // insert into index 1 when storage length is 0
        storage.insert(&id, 5);
    }

    #[test]
    fn insert_to_update_value() {
        let mut allocator = Allocator::<TestId>::new();
        let mut storage = IndexedVec::<TestId, u32>::new();

        let id = allocator.create_entity();

        storage.insert(&id, 2);
        storage.insert(&id, 3);

        assert_eq!(Some(&3), storage.get(&id));
    }
}
