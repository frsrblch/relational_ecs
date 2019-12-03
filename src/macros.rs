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
    
        impl Link<$id_a, $id_b> for State {}
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
    
        impl Remove<$id_a, $id_b> for State {
            fn remove(&mut self, id: &VerifiedEntity<$id_a>, value: $id_b) -> Option<$id_b> {
                let values = &mut self.$field_a[id];
                values.remove(&value)
            }
        }
    
        impl Insert<$id_b, $id_a> for State {
            fn insert(&mut self, id: &VerifiedEntity<$id_b>, value: $id_a) {
                self.$field_b.insert(id, value);
            }
        }
    
        impl Link<$id_a, $id_b> for State {}
    }
}