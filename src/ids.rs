use crate::entities::Generation;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::allocators::FlexAllocator;
use std::cmp::Ordering;
use std::fmt::{Formatter, Error, Display};

pub trait IdIndex<T>: Display {
    fn index(&self) -> usize;
}

#[derive(Debug)]
pub struct Id<T> {
    pub (crate) index: u32,
    marker: PhantomData<T>,
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Id({})", self.index)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<T> Id<T> {
    pub (crate) fn new(index: u32) -> Self {
        Self {
            index,
            marker: PhantomData,
        }
    }
}

impl<T> IdIndex<T> for Id<T> {
    fn index(&self) -> usize {
        self.index as usize
    }
}

#[derive(Debug)]
pub struct GenId<T> {
    pub (crate) id: Id<T>,
    pub (crate) gen: Generation,
}

impl<T> Display for GenId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "GenId({},{})", self.id.index, self.gen)
    }
}

impl<T> Clone for GenId<T> {
    fn clone(&self) -> Self {
        Self::new(self.id.index, self.gen)
    }
}

impl<T> Copy for GenId<T> {}

impl<T> GenId<T> {
    pub (crate) fn new(index: u32, gen: Generation) -> Self {
        Self {
            id: Id::<T>::new(index),
            gen,
        }
    }
}

impl<T> PartialEq for GenId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.gen.eq(&other.gen)
    }
}

impl<T> Eq for GenId<T> {}

impl<T> Hash for GenId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialOrd for GenId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for GenId<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug)]
pub struct ValidGenId<'a, T> {
    pub id: GenId<T>,
    pub (crate) marker: PhantomData<&'a FlexAllocator<T>>,
}

impl<'a, T> ValidGenId<'a, T> {
    pub fn new(id: GenId<T>) -> Self {
        Self {
            id,
            marker: PhantomData
        }
    }
}

impl<'a, T> Display for ValidGenId<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.id.fmt(f)
    }
}

impl<'a, T> IdIndex<T> for ValidGenId<'a, T> {
    fn index(&self) -> usize {
        self.id.id.index()
    }
}