use relational_ecs::storage::Component;
use relational_ecs::traits::Split;
use relational_ecs::allocators::*;
use relational_ecs::ids::*;
use relational_ecs::traits_new::*;

fn main() {
    let mut state = State::default();
    let (ints, ids) = state.split();

    let vid1 = ints.create(Coords { x: 2, y: 3 }, ids);
//    let id1 = vid1.id;
    let id1 = vid1;

    let vid2 = ints.create(Coords { x: 0, y: 0 }, ids);
//    let id2 = vid2.id;
    let id2 = vid2;

    // doesn't compile because the lifetimes of id1 and id2 cannot overlap:
//    dbg!(vid1, vid2);
    dbg!(id1, id2);
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub ints: Ints,
    pub ids: FixedAllocator<Ints>,
}

impl Create<'_, Ints, Coords> for Ints {
    type Allocator = FixedAllocator<Ints>;

    fn create(&mut self, value: Coords, allocator: &mut FixedAllocator<Ints>) -> Id<Ints> {
        let id = allocator.create();
        self.insert(&id, value);
        id
    }
}

impl Split<Ints, FixedAllocator<Ints>> for State {
    fn split(&mut self) -> (&mut Ints, &mut FixedAllocator<Ints>) {
        (&mut self.ints, &mut self.ids)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ints {
    pub x: Component<Ints, u32>,
    pub y: Component<Ints, u32>,
}

impl Insert<Ints, Coords> for Ints {
    fn insert(&mut self, id: &impl IdIndex<Ints>, value: Coords) {
        self.x.insert(id, value.x);
        self.y.insert(id, value.y);
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Coords {
    pub x: u32,
    pub y: u32
}

