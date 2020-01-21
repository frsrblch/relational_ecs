use relational_ecs::storage::Component;
use relational_ecs::allocators::{FlexAllocator, Allocator};
use relational_ecs::traits::Split;
use relational_ecs::ids::{IdIndex, ValidGenId};

fn main() {
    let mut state = State::default();

    let vid1 = state.create(Coords { x: 2, y: 3 });
    let id1 = vid1.id;

    let vid2 = state.create(Coords { x: 0, y: 0 });
    let id2 = vid2.id;

    // doesn't compile because the lifetimes of id1 and id2 cannot overlap:
//    dbg!(vid1, vid2);
    dbg!(id1, id2);


}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub ints: Ints,
    pub ids: FlexAllocator<Ints>,
}

impl State {
    pub fn create(&mut self, value: Coords) -> ValidGenId<Ints> {
        let (ints, ids) = self.split();
        let id = ids.create();
        ints.insert(&id, value);
        id
    }
}

impl Split<Ints, FlexAllocator<Ints>> for State {
    fn split(&mut self) -> (&mut Ints, &mut FlexAllocator<Ints>) {
        (&mut self.ints, &mut self.ids)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ints {
    pub x: Component<Ints, u32>,
    pub y: Component<Ints, u32>,
}

impl Ints {
    pub fn insert(&mut self, id: &impl IdIndex<Ints>, value: Coords) {
        self.x.insert(id, value.x);
        self.y.insert(id, value.y);
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Coords {
    pub x: u32,
    pub y: u32
}

