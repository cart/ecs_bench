use super::{Position, Velocity, N_POS, N_POS_VEL};
use criterion::{measurement::WallTime, BenchmarkGroup};
use legion_experimental::{
    storage::{GroupSource, PackOptions},
    IntoQuery, Read, Universe, World, WorldOptions, Write,
};

fn build() -> World {
    let universe = Universe::new();
    let mut world = universe.create_world();

    world.extend(
        (0..N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 })),
    );
    world.extend((0..N_POS - N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 },)));

    world
}

fn build_packed() -> World {
    let mut world = World::with_options(WorldOptions {
        groups: vec![<(Position, Velocity)>::to_group()],
    });

    world.extend(
        (0..N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 })),
    );
    world.extend((0..N_POS - N_POS_VEL).map(|_| (Position { x: 0.0, y: 0.0 },)));

    world.pack(PackOptions::force());

    world
}


pub fn bench(group: &mut BenchmarkGroup<WallTime>) {
    let mut world = build();
    let mut query = <(Write<Position>, Read<Velocity>)>::query();

    group.bench_function("legion_experimental", |b| {
        b.iter(|| {
            for (mut pos, vel) in query.iter_mut(&mut world) {
                pos.x += vel.dx;
                pos.y += vel.dy;
            }
        })
    });

    let mut world = build_packed();
    let mut query = <(Write<Position>, Read<Velocity>)>::query();
    group.bench_function("legion_experimental_packed", |b| {
        b.iter(|| {
            for (mut pos, vel) in query.iter_mut(&mut world) {
                pos.x += vel.dx;
                pos.y += vel.dy;
            }
        })
    });
}


pub fn bench_build(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("legion_experimental", |b| {
        b.iter(|| {
            build();
        })
    });

    group.bench_function("legion_experimental_packed", |b| {
        b.iter(|| {
            build_packed();
        })
    });
}
