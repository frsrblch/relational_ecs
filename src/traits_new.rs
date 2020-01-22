use std::fmt::Display;

pub trait IdIndex<T> {
    type Id: Display;
    fn index(&self) -> usize;
    fn id(&self) -> Self::Id;
}

pub trait Allocator<'a, T> {
    type Id: IdIndex<T>;
    fn create(&'a mut self) -> Self::Id;
}

pub trait Arena<'a>: Sized {
    type Row;
    type Allocator: Allocator<'a, Self>;
}

pub trait Insert<'a>: Arena<'a> {
    fn insert(&mut self, id: &impl IdIndex<Self>, value: Self::Row);
}

pub trait Create<'a>: Insert<'a> {
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

impl<'a, T: Insert<'a>> Create<'a> for T {}

//pub trait Construct<ID, T> {
//    type Id: IdIndex<ID>;
//    fn construct(&mut self, value: T) -> <Self::Id as IdIndex<ID>>::Id;
//}
//
//pub trait Link<IdA, IdB> {
//    fn link(&mut self, id_a: &IdA, id_b: &IdB);
//}