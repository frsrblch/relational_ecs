use criterion::*;
use relational_ecs::prelude::*;

id_type!(TestId);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position(f32);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Rotation(f32);

#[derive(Debug, Default)]
pub struct World {
    allocator: Allocator<TestId>,

    position: IndexedVec<TestId, Position>,
    rotation: IndexedVec<TestId, Rotation>,
}

impl World {
    fn insert(&mut self, pos: Position, rot: Rotation) {
        let id = self.allocator.create_entity();

        self.position.insert(&id, pos);
        self.rotation.insert(&id, rot);
    }
}

fn setup(n: usize) -> World {
    let mut world = World::default();

    for _ in 0..n {
        world.insert(Position(0.0), Rotation(0.0));
    }

    world
}

fn bench_iter_simple(c: &mut Criterion) {
    c.bench_function("iter-simple", |b| {
        let mut world = setup(2000);

        b.iter(|| {
            for (pos, rot) in world.position.values.iter().zip(world.rotation.values.iter_mut()) {
                rot.0 = pos.0;
            }
        })
    });
}

fn bench_iter_ids(c: &mut Criterion) {
    c.bench_function("iter-simple-ids", |b| {
        let mut world = setup(2000);

        b.iter(|| {
            let allocator = &world.allocator;
            let position = &world.position;
            let rotation = &mut world.rotation;
            allocator.ids()
                .for_each(move |id| {
                    rotation[&id].0 = position[&id].0;
                });
        })
    });
}

criterion_group!(
    basic,
//    bench_create_delete,
    bench_iter_simple,
    bench_iter_ids,
//    bench_iter_complex,
//    bench_iter_chunks_simple,
//    bench_iter_chunks_complex
);
criterion_main!(basic);