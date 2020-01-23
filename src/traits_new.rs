use std::fmt::{Display, Debug};

pub trait IdIndex {
    type Arena;
    type Id: Display;

    fn index(&self) -> usize;
    fn id(&self) -> Self::Id;
}

pub trait Allocator<T> {
    type Id: IdIndex<Arena=T>;

    fn create(&mut self) -> &Self::Id;
}

pub trait Arena: Sized + Clone + Default + Debug {
    type Id: IdIndex<Arena=Self>;
    type Row;
    type Allocator: Allocator<Self>;

    fn insert(&mut self, id: &<Self::Allocator as Allocator<Self>>::Id, value: Self::Row);

    fn create<'a>(
        &mut self,
        value: Self::Row,
        allocator: &'a mut Self::Allocator
    ) -> &'a <Self::Allocator as Allocator<Self>>::Id {
        let id = allocator.create();
        self.insert(&id, value);
        id
    }
}

pub trait Construct<ID, T> {
    type Id: IdIndex<Arena=ID>;

    fn construct(&mut self, value: T) -> Self::Id;
}

pub trait Link<A, B>
    where
        A: Arena,
        B: Arena,
{
    fn link(
        &mut self,
        a: &A::Id,
        b: &B::Id);
}

pub trait Update<T> {
    fn update(state: &mut T);
}

pub trait  Split<E, S> {
    fn split(&mut self) -> (&mut E, &mut S);
}