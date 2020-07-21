use super::{Position, Velocity, N_POS, N_POS_VEL};
use bevy_ecs::Resources;
use bevy_ecs::{Bundle, Entity, IntoForEachSystem, IntoQuerySystem, Query, World, Mut};
use criterion::{measurement::WallTime, BenchmarkGroup};

#[derive(Bundle)]
struct PosVel {
    pos: Position,
    vel: Velocity,
}

#[derive(Bundle)]
struct Pos {
    pos: Position,
}

fn build() -> World {
    let mut world = World::new();
    world.spawn_batch((0..N_POS - N_POS_VEL).map(|_| Pos {
        pos: Position { x: 0.0, y: 0.0 },
    }));
    world.spawn_batch((0..N_POS_VEL).map(|_| PosVel {
        pos: Position { x: 0.0, y: 0.0 },
        vel: Velocity { dx: 1.0, dy: 1.0 },
    }));
    world
}

fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query.iter() {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

fn move_system_entity(mut query: Query<(Entity, &mut Position, &Velocity)>) {
    for (_entity, mut pos, vel) in &mut query.iter() {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

fn move_system_foreach(mut pos: Mut<Position>, vel: &Velocity) {
    pos.x += vel.dx;
    pos.y += vel.dy;
}

fn move_system_foreach_entity(_entity: Entity, mut pos: Mut<Position>, vel: &Velocity) {
    pos.x += vel.dx;
    pos.y += vel.dy;
}

pub fn bench(group: &mut BenchmarkGroup<WallTime>) {
    let world = build();
    let resources = Resources::default();
    let mut system = move_system.system();
    group.bench_function("bevy", |b| b.iter(|| system.run(&world, &resources)));

    let world = build();
    let resources = Resources::default();
    let mut system = move_system_entity.system();
    group.bench_function("bevy_entity", |b| b.iter(|| system.run(&world, &resources)));

    let world = build();
    let resources = Resources::default();
    let mut system = move_system_foreach.system();
    group.bench_function("bevy_foreach", |b| {
        b.iter(|| system.run(&world, &resources))
    });

    let world = build();
    let resources = Resources::default();
    let mut system = move_system_foreach_entity.system();
    group.bench_function("bevy_foreach_entity", |b| {
        b.iter(|| system.run(&world, &resources));
    });
}

pub fn bench_build(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("bevy", |b| {
        b.iter(|| build());
    });
}
