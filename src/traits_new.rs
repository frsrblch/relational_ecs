use std::fmt::Display;

pub trait IdIndex {
    type Arena;
    type Id: Display + Sized;
    fn index(&self) -> usize;
    fn id(&self) -> Self::Id;
}

pub trait Allocator<'a, T> {
    type Id: IdIndex<Arena=T>;
    fn create(&'a mut self) -> Self::Id;
}

pub trait Arena<'a>: Sized {
    type Id: IdIndex<Arena=Self>;
    type Row;
    type Allocator: Allocator<'a, Self>;

    fn insert(&mut self, id: &<<Self as Arena<'a>>::Allocator as Allocator<'a, Self>>::Id, value: Self::Row);
}

pub trait Create<'a>: Arena<'a> {
    fn create(
        &mut self,
        value: Self::Row,
        allocator: &'a mut Self::Allocator
    ) -> <Self::Allocator as Allocator<'a, Self>>::Id {
        let id = allocator.create();
        self.insert(&id, value);
        id
    }
}

impl<'a, T: Arena<'a>> Create<'a> for T {}

pub trait Construct<ID, T> {
    type Id: IdIndex<Arena=ID> + Sized;
    fn construct(&mut self, value: T) -> Self::Id;
}

pub trait Link<A, B>
    where
        A: IdIndex,
        B: IdIndex,
{
    fn link(&mut self, a: &A, b: &B);
}

