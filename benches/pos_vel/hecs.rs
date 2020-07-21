use criterion::{measurement::WallTime, BenchmarkGroup};
use super::{Position, Velocity, N_POS, N_POS_VEL};
use hecs::{Bundle, World};

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

pub fn bench(group: &mut BenchmarkGroup<WallTime>) {
    let world = build();

    group.bench_function("hecs", |b| {
        b.iter(|| {
            for (_, (pos, vel)) in &mut world.query::<(&mut Position, &Velocity)>() {
                pos.x += vel.dx;
                pos.y += vel.dy;
            }
        })
    });
}

pub fn bench_build(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("hecs", |b| {
        b.iter(|| build());
    });
}
