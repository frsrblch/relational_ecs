use crate::entities::Generation;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::cmp::Ordering;
use std::fmt::{Formatter, Error, Display};
use crate::traits_new::IdIndex;

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

impl<T> IdIndex for Id<T> {
    type Arena = T;
    type Id = Self;

    fn index(&self) -> usize {
        self.index as usize
    }

    fn id(&self) -> Self::Id {
        *self
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
            id: Id::new(index),
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
pub struct Valid<T> {
    pub id: GenId<T>,
}

impl<T> Valid<T> {
    pub fn new(id: GenId<T>) -> Self {
        Self {
            id,
        }
    }
}

impl<T> IdIndex for Valid<T> {
    type Arena = T;
    type Id = GenId<T>;

    fn index(&self) -> usize {
        self.id.id.index()
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}