use std::fmt::Display;

pub trait IdIndex<T>: Display {
    type Id;
    fn index(&self) -> usize;
    fn id(&self) -> Self::Id;
}

pub trait Allocator<'a, T> {
    type Id: IdIndex<T>;
    fn create(&'a mut self) -> Self::Id;
}

pub trait Arena<'a>: Sized {
    type Allocator: Allocator<'a, Self>;
}

pub trait Insert<T>: Sized {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: T);
}

pub trait Create<'a, ID, T>: Insert<T> + Arena<'a>
    where <<Self as Arena<'a>>::Allocator as Allocator<'a, Self>>::Id: IdIndex<Self>
{
    fn create(&mut self, value: T, allocator: &'a mut Self::Allocator) -> <Self::Allocator as Allocator<'a, Self>>::Id {
        let id = allocator.create();
        self.insert(&id, value);
        id
    }
}

pub trait Construct<ID, T> {
    type Id: IdIndex<ID>;
    fn construct(&mut self, value: T) -> Self::Id;
}

pub trait Link<IdA, IdB> {
    fn link(&mut self, id_a: &IdA, id_b: &IdB);
}