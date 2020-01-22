use crate::traits_new::*;
use std::marker::PhantomData;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Component<ID, T> {
    values: Vec<T>,
    marker: PhantomData<ID>,
}

impl<ID, T> Default for Component<ID, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<'a, ID: Arena<'a>, T> Component<ID, T> {
    pub fn new() -> Self { Default::default() }

    pub fn insert(&mut self, id: &<ID::Allocator as Allocator<'a, ID>>::Id, value: T) {
        match id.index() {
            index if index < self.values.len() => self.values[index] = value,
            index if index == self.values.len() => self.values.push(value),
            _ => { panic!("Given index is invalid: {}", id.id()) }
        }
    }

    pub fn get(&self, id: &<ID::Allocator as Allocator<'a, ID>>::Id) -> &T {
        self.values.get(id.index()).unwrap()
    }

    pub fn get_mut(&mut self, id: &<ID::Allocator as Allocator<'a, ID>>::Id) -> &mut T {
        self.values.get_mut(id.index()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocators::{FixedAllocator, Allocator, FlexAllocator};

    #[derive(Debug)]
    struct Test;

    #[test]
    #[should_panic]
    fn insert_given_invalid_id_panics() {
        let mut allocator = FixedAllocator::<Test>::default();
        let mut component = Component::<Test, u32>::new();

        let _id0 = allocator.create();
        let id1 = allocator.create();

        component.insert(&id1, 0);
    }

    #[test]
    fn insert_and_retrieve() {
        let mut allocator = FixedAllocator::<Test>::default();
        let mut component = Component::<Test, u32>::new();

        let id = allocator.create();
        component.insert(&id, 3);

        assert_eq!(&3, component.get(&id));
    }

    #[test]
    fn reuse_index() {
        let mut allocator = &mut FlexAllocator::<Test>::default();
        let mut component = Component::<Test, u32>::new();

        let id_0_1 = allocator.create();
        component.insert(&id_0_1, 2);
        allocator.kill(id_0_1.id);

        let id_0_2 = allocator.create();
        component.insert(&id_0_2, 3);

        assert_eq!(id_0_1.id.id, id_0_2.id.id); // same index
        assert_ne!(id_0_1.id.gen, id_0_2.id.gen); // different gen
        assert_eq!(&3, component.get(&id_0_2));
    }
}