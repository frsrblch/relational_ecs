use crate::traits_new::*;
use std::fmt::Debug;
use rustc_hash::FxHashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct ComponentMap<ID, T>
    where
        ID: Arena,
        <ID::Id as IdIndex>::Id: Hash + Clone + Eq + Debug,
{
    map: FxHashMap<<ID::Id as IdIndex>::Id, T>,
}

impl<ID, T> Default for ComponentMap<ID, T>
    where
        ID: Arena,
        <ID::Id as IdIndex>::Id: Hash + Clone + Eq + Debug,
{
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }
}

impl<ID, T> ComponentMap<ID, T>
    where
        ID: Arena,
        <ID::Id as IdIndex>::Id: Hash + Clone + Eq + Debug,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, id: &ID::Id, value: T) {
        self.map.insert(id.id(), value);
    }

    pub fn get(&self, id: &ID::Id) -> Option<&T> {
        self.map.get(&id.id())
    }

    pub fn remove(&mut self, id: &ID::Id) -> Option<T> {
        self.map.remove(&id.id())
    }
}