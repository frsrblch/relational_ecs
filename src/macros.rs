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
