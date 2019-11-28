#[macro_export]
macro_rules! id_type {
($type_name:ident) => {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
    pub struct $type_name(u32, Generation);

    impl IdType for $type_name {
        fn new(index: u32) -> Self {
            Self(index, Generation::default())
        }

        fn create(index: usize, gen: Generation) -> Self {
            Self(index as u32, gen)
        }

        fn index(&self) -> usize {
            self.0 as usize
        }

        fn generation(&self) -> Generation {
            self.1
        }
    }
};
}
