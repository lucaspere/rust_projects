use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sixty_challenge_days::impls::dsa::arrays::{
    rotate_3_way_reverses, rotate_by_juggling, rotate_left,
};

#[inline]
fn create_array(array_size: usize) -> Vec<u32> {
    let array: Vec<u32> = (0..array_size as u32).collect();

    array
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(40))
}

pub fn rotate_array_algorithms(c: &mut Criterion) {
    let mut array = create_array(10000);
    let mid = 20;
    let mut rotation_group = c.benchmark_group("Rotation Array Algorithms");
    rotation_group.bench_function("rotate_one_by_one", |b| {
        b.iter(|| {
            rotate_left(black_box(&mut array), mid);
        });
    });

    rotation_group.bench_function("rotate_reversal", |b| {
        b.iter(|| {
            rotate_3_way_reverses(black_box(&mut array), mid);
        });
    });

    rotation_group.bench_function("rotate_juggling", |b| {
        b.iter(|| {
            rotate_by_juggling(black_box(&mut array), mid);
        });
    });

    rotation_group.finish();
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = rotate_array_algorithms
);

criterion_main!(benches);
