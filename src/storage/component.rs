use crate::traits_new::*;
use std::marker::PhantomData;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Component<ID: Arena, T> {
    pub values: Vec<T>,
    marker: PhantomData<ID>,
}

impl<ID: Arena, T> Default for Component<ID, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<'a, ID: Arena, T> Component<ID, T> {
    pub fn new() -> Self { Default::default() }

    pub fn insert(&mut self, id: &<ID::Allocator as Allocator<ID>>::Id, value: T) {
        match id.index() {
            index if index < self.values.len() => self.values[index] = value,
            index if index == self.values.len() => self.values.push(value),
            _ => { panic!("Given index is invalid: {}", id.id()) }
        }
    }

    pub fn get(&self, id: &<ID::Allocator as Allocator<ID>>::Id) -> &T {
        self.values.get(id.index()).unwrap()
    }

    pub fn get_mut(&mut self, id: &<ID::Allocator as Allocator<ID>>::Id) -> &mut T {
        self.values.get_mut(id.index()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocators::{FixedAllocator, GenAllocator};
    use crate::traits_new::Allocator;
    use crate::ids::{Id, GenId, Valid};

    #[derive(Debug)]
    struct Fixed;

    impl Arena for Fixed {
        type Id = Id<Self>;
        type Row = ();
        type Allocator = FixedAllocator<Self>;

        fn insert(&mut self, id: &Self::Id, value: Self::Row) {
            unimplemented!()
        }
    }

    #[derive(Debug)]
    struct Gen;

    impl Arena for Gen {
        type Id = Valid<Self>;
        type Row = ();
        type Allocator = GenAllocator<Self>;

        fn insert(&mut self, id: &Self::Id, value: Self::Row) {
            unimplemented!()
        }
    }

    #[test]
    #[should_panic]
    fn insert_given_invalid_id_panics() {
        let mut allocator = FixedAllocator::<Fixed>::default();
        let mut component = Component::<Fixed, u32>::new();

        let _id0 = allocator.create();
        let id1 = allocator.create();

        component.insert(&id1, 0);
    }

    #[test]
    fn insert_and_retrieve() {
        let mut allocator = FixedAllocator::<Fixed>::default();
        let mut component = Component::<Fixed, u32>::new();

        let id = allocator.create();
        component.insert(id, 3);

        assert_eq!(&3, component.get(&id));
    }

    #[test]
    fn reuse_index() {
        let mut allocator = GenAllocator::<Gen>::default();
        let mut component = Component::<Gen, u32>::new();

        let id_0_1 = allocator.create();
        component.insert(id_0_1, 2);
        let id_0_1 = id_0_1.id;
        allocator.kill(id_0_1);

        let id_0_2 = allocator.create();
        component.insert(id_0_2, 3);

        assert_eq!(id_0_1.id.index, id_0_2.id.id.index); // same index
        assert_ne!(id_0_1.gen, id_0_2.id.gen); // different gen
        assert_eq!(&3, component.get(&id_0_2));
    }
}