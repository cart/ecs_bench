use super::{Position, Velocity, N_POS, N_POS_VEL_MODULUS};
use criterion::{measurement::WallTime, BenchmarkGroup};
use specs::prelude::*;

impl Component for Position {
    type Storage = VecStorage<Position>;
}

impl Component for Velocity {
    type Storage = VecStorage<Velocity>;
}

struct VelSys;
impl<'a> System<'a> for VelSys {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);
    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        (&mut pos, &vel).join().for_each(|(p, v)| {
            p.x += v.dx;
            p.y += v.dy;
        });
    }
}

fn build() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    // setup entities
    {
        for i in 0..N_POS {
            let mut builder = world.create_entity().with(Position { x: 0.0, y: 0.0 });
            if i % N_POS_VEL_MODULUS == 0 {
                builder = builder.with(Velocity { dx: 1.0, dy: 1.0 });
            }

            builder.build();
        }
    }

    world
}

pub fn bench_build(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("specs", |b| b.iter(|| build()));
}

pub fn bench(group: &mut BenchmarkGroup<WallTime>) {
    let mut world = build();
    let mut sys = VelSys;
    System::setup(&mut sys, &mut world);
    group.bench_function("specs", |b| {
        b.iter(|| {
            sys.run_now(&world);
        })
    });
}
