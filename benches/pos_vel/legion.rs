use super::{Position, Velocity, N_POS, N_POS_VEL};
use legion::prelude::*;
use criterion::{measurement::WallTime, BenchmarkGroup};

fn build() -> World {
    let universe = Universe::new();
    let mut world = universe.create_world();

    world.insert(
        (),
        (0..N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 })),
    );
    world.insert(
        (),
        (0..N_POS - N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 },)),
    );

    world
}

pub fn bench(group: &mut BenchmarkGroup<WallTime>) {
    let mut world = build();
    let query = <(Write<Position>, Read<Velocity>)>::query();

    group.bench_function("legion", |b| {
        b.iter(|| {
        for (mut pos, vel) in query.iter_mut(&mut world) {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
        })
    });
}

pub fn bench_build(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("legion", |b| b.iter(|| build()));
}
