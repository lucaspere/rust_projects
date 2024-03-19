use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sixty_challenge_days::impls::dsa::recursion;

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(230))
}

pub fn fibonnaci(c: &mut Criterion) {
    let mut fibonnaci_group = c.benchmark_group("Fibonnaci algorithms");
    fibonnaci_group.bench_function("fibonnaci", |b| {
        b.iter(|| recursion::fibonnaci(black_box(43)));
    });

    fibonnaci_group.bench_function("fibonnaci memoization", |b| {
        b.iter(|| {
            recursion::fibonnaci_mem()(black_box(43));
        });
    });

    fibonnaci_group.finish();
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = fibonnaci
);

criterion_main!(benches);
