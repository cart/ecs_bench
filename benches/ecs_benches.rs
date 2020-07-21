pub mod pos_vel;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use hashbrown::HashMap;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("pos_vel");
    pos_vel::bevy::bench(&mut group);
    pos_vel::legion::bench(&mut group);
    pos_vel::legion_experimental::bench(&mut group);
    pos_vel::specs::bench(&mut group);
    pos_vel::hecs::bench(&mut group);
    pos_vel::shipyard::bench(&mut group);
    group.finish();

    let mut group = c.benchmark_group("build");
    pos_vel::bevy::bench_build(&mut group);
    pos_vel::legion::bench_build(&mut group);
    pos_vel::legion_experimental::bench_build(&mut group);
    pos_vel::specs::bench_build(&mut group);
    pos_vel::hecs::bench_build(&mut group);
    pos_vel::shipyard::bench_build(&mut group);
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
