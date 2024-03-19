use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use sixty_challenge_days::impls::dsa::arrays::*;

#[inline]
fn create_array(shuffled: bool, random: bool) -> Vec<u32> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    const ARRAY_SIZE: usize = 5000;
    const SWAPS: usize = 1000;

    let mut rng = thread_rng();
    let mut array: Vec<u32> = (0..ARRAY_SIZE as u32).collect();

    // Shuffle the array to make it mostly ordered
    if shuffled {
        array.shuffle(&mut rng);
    }

    if random {
        for _ in 0..SWAPS {
            let i = rng.gen_range(0..ARRAY_SIZE);
            let j = rng.gen_range(0..ARRAY_SIZE);
            array.swap(i, j);
        }
    } else {
        array.reverse()
    }
    // Perform some random swaps to make it partially ordered

    array
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(40))
}

pub fn insertion_sort_shuffle(c: &mut Criterion) {
    let mut array = create_array(true, true);
    let mut insertion_sort_group = c.benchmark_group("Insertion Sort Randomly Smaples");
    insertion_sort_group.bench_function("insertion_sort", |b| {
        b.iter(|| {
            insertion_sort(black_box(&mut array));
        });
    });

    insertion_sort_group.bench_function("insertion_sort_optimization", |b| {
        b.iter(|| {
            insertion_sort_optimization(black_box(&mut array));
        });
    });

    insertion_sort_group.finish();
}

pub fn insertion_sort_worst_case_reverse(c: &mut Criterion) {
    let mut array = create_array(false, false);
    let mut insertion_sort_group = c.benchmark_group("Insertion Sort Reversed Samples");
    insertion_sort_group.bench_function("insertion_sort", |b| {
        b.iter(|| {
            insertion_sort(black_box(&mut array));
        });
    });

    insertion_sort_group.bench_function("insertion_sort_optimization", |b| {
        b.iter(|| {
            insertion_sort_optimization(black_box(&mut array));
        });
    });

    insertion_sort_group.finish();
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = insertion_sort_shuffle, insertion_sort_worst_case_reverse
);

criterion_main!(benches);
