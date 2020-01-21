use crate::ids::IdIndex;
use crate::allocators::Allocator;

pub trait Insert<ID, T> {
    fn insert(&mut self, id: &impl IdIndex<ID>, value: T);
}

pub trait Create<'a, ID, T> {
    type Allocator: Allocator<'a, ID>;

    fn create(&mut self, value: T, allocator: &'a mut Self::Allocator) -> <Self::Allocator as Allocator<'a, ID>>::Id;
}