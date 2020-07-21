#![allow(dead_code)]
#![feature(test)]
extern crate test;

use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_VEL};
use test::Bencher;
use tiny_ecs::Entities;

fn build() -> Entities {
    let mut entities = Entities::new(Some(N_POS), Some(2));

    for _ in 0..N_POS - N_POS_VEL {
        entities
            .new_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .unwrap()
            .finalise()
            .unwrap();
    }
    for _ in 0..N_POS_VEL {
        entities
            .new_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .unwrap()
            .with(Velocity { dx: 1.0, dy: 1.0 })
            .unwrap()
            .finalise()
            .unwrap();
    }
    entities
}

fn bench_update_safe(b: &mut Bencher) {
    let world = build();

    let vel = world.borrow::<Velocity>().unwrap();
    let mut pos = world.borrow_mut::<Position>().unwrap();

    b.iter(|| {
        for (id, vel) in vel.iter() {
            if let Some(pos) = pos.get_mut(id) {
                pos.x += vel.dx;
                pos.y += vel.dy;
            }
        }
    });
}

fn bench_update_unsafe(b: &mut Bencher) {
    let world = build();

    let vel = unsafe { world.borrow_unchecked::<Velocity>().unwrap() };
    let pos = unsafe { world.borrow_mut_unchecked::<Position>().unwrap() };

    b.iter(|| {
        for (id, vel) in vel.iter() {
            if let Some(pos) = pos.get_mut(id) {
                pos.x += vel.dx;
                pos.y += vel.dy;
            }
        }
    });
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    bench_update_safe(b);
}
