use super::{Position, Velocity, N_POS, N_POS_VEL};
use criterion::{measurement::WallTime, BenchmarkGroup};
use shipyard::*;

fn build(world: &World) {
    world.run(
        |mut entities: EntitiesViewMut,
         mut positions: ViewMut<Position>,
         mut velocities: ViewMut<Velocity>| {
            for _ in 0..N_POS - N_POS_VEL {
                entities.add_entity(&mut positions, Position { x: 0.0, y: 0.0 });
            }

            for _ in 0..N_POS_VEL {
                entities.add_entity(
                    (&mut positions, &mut velocities),
                    (Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 }),
                );
            }
        },
    );
}

pub fn bench(group: &mut BenchmarkGroup<WallTime>) {
    // normal
    // {
        // let world = World::new();
        // build(&world);
        // group.bench_function("shipyard", |b| {
        //     b.iter(|| {
        //         world.run(
        //             |mut positions: ViewMut<Position>, velocities: View<Velocity>| {
        //                 for (pos, vel) in (&mut positions, &velocities).iter() {
        //                     pos.x += vel.dx;
        //                     pos.y += vel.dy;
        //                 }
        //             },
        //         )
        //     })
        // });
    // }

    // packed
    {
        let world = World::new();
        world.run(
            |mut positions: ViewMut<Position>, mut velocities: ViewMut<Velocity>| {
                (&mut positions, &mut velocities).tight_pack();
            },
        );
        build(&world);

        group.bench_function("shipyard-packed", |b| {
            b.iter(|| {
                world.run(
                    |mut positions: ViewMut<Position>, velocities: View<Velocity>| {
                        for (pos, vel) in (&mut positions, &velocities).iter() {
                            pos.x += vel.dx;
                            pos.y += vel.dy;
                        }
                    },
                )
            })
        });
    }
}

pub fn bench_build(group: &mut BenchmarkGroup<WallTime>) {
    group.bench_function("shipyard", |b| {
        b.iter(|| {
            let world = World::new();
            build(&world);
        })
    });
}
