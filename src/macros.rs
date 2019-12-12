#[macro_export]
macro_rules! id_type {
    ($type_name:ident) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, std::hash::Hash, Ord, PartialOrd)]
        pub struct $type_name(u32, $crate::entities::Generation);

        impl $crate::traits::IdType for $type_name {
            fn new(index: u32) -> Self {
                Self(index, $crate::entities::Generation::default())
            }

            fn create(index: usize, gen: $crate::entities::Generation) -> Self {
                Self(index as u32, gen)
            }

            fn index(&self) -> usize {
                self.0 as usize
            }

            fn generation(&self) -> $crate::entities::Generation {
                self.1
            }
        }
    };
}

#[macro_export]
macro_rules! link {
    ($id_a:ty, $field_a:ident, $id_b:ty, $field_b:ident) => {
        impl Get<$id_a, $id_b> for State {
            fn get(&self, id: &VerifiedEntity<$id_a>) -> Option<&$id_b> {
                self.$field_a.get(id)
            }
            fn get_mut(&mut self, id: &VerifiedEntity<$id_a>) -> Option<&mut $id_b> {
                self.$field_a.get_mut(id)
            }
        }

        impl Get<$id_b, $id_a> for State {
            fn get(&self, id: &VerifiedEntity<$id_b>) -> Option<&$id_a> {
                self.$field_b.get(id)
            }
            fn get_mut(&mut self, id: &VerifiedEntity<$id_b>) -> Option<&mut $id_a> {
                self.$field_b.get_mut(id)
            }
        }

        impl Insert<$id_b, $id_a> for State {
            fn insert(&mut self, id: &VerifiedEntity<$id_b>, value: $id_a) {
                self.$field_b.insert(id, value);
            }
        }
    
        impl Insert<$id_a, $id_b> for State {
            fn insert(&mut self, id: &VerifiedEntity<$id_a>, value: $id_b) {
                self.$field_a.insert(id, value);
            }
        }

        impl Lookup<'_, $id_a, $id_b> for State {}
        impl Lookup<'_, $id_b, $id_a> for State {}
    
        impl Link<$id_a, $id_b> for State {
            fn link(&mut self, a: &VerifiedEntity<$id_a>, b: &VerifiedEntity<$id_b>) {
                self.insert(a, b.entity);
                self.insert(b, a.entity);
            }
        }
    }
}

#[macro_export]
macro_rules! link_to_many {
    ($id_a:ty, $field_a:ident, $id_b:ty, $field_b:ident) => {
        impl Insert<$id_a, $id_b> for State {
            fn insert(&mut self, id: &VerifiedEntity<$id_a>, value: $id_b) {
                let locations = &mut self.$field_a[id];
                locations.insert(value);
            }
        }
    
        impl RemoveFrom<$id_a, $id_b> for State {
            fn remove_from(&mut self, id: &VerifiedEntity<$id_a>, value: $id_b) -> Option<$id_b> {
                let values = &mut self.$field_a[id];
                values.remove(&value)
            }
        }
    
        impl Insert<$id_b, $id_a> for State {
            fn insert(&mut self, id: &VerifiedEntity<$id_b>, value: $id_a) {
                self.$field_b.insert(id, value);
            }
        }
    
        impl Link<$id_a, $id_b> for State {
            fn link(&mut self, a: &VerifiedEntity<$id_a>, b: &VerifiedEntity<$id_b>) {
                self.insert(a, b.entity);
                self.insert(b, a.entity);
            }
        }
    }
}