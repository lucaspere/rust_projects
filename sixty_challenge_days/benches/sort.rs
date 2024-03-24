use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use sixty_challenge_days::impls::dsa::sort::{
    insertion_sort, insertion_sort_optimization, quick_sort_hoare, quick_sort_middle_three,
    quick_sort_random, quick_sort_with_custom_part,
};

#[inline]
fn create_array(shuffled: bool, random: bool) -> Vec<u32> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    const ARRAY_SIZE: usize = 5000;
    const SWAPS: usize = 1000;

    let mut rng = thread_rng();
    let mut array: Vec<u32> = (0..ARRAY_SIZE as u32).collect();

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
    }

    array
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::from_secs(100))
}

pub fn insertion_sort_average_case(c: &mut Criterion) {
    let mut sorted_array: Vec<u32> = create_array(true, true);
    let mut sort_group = c.benchmark_group("Insertion Sort Average Case");
    sort_group.bench_function("insertion_sort_average_case", |b| {
        b.iter(|| {
            insertion_sort(black_box(&mut sorted_array));
        });
    });

    sort_group.bench_function("insertion_sort_optimization_average_case", |b| {
        b.iter(|| {
            insertion_sort_optimization(black_box(&mut sorted_array));
        });
    });

    sort_group.finish()
}

pub fn insertion_sort_worst_case(c: &mut Criterion) {
    let mut reversed_array: Vec<u32> = create_array(false, false);
    let mut sort_group = c.benchmark_group("Insertion Sort Worst Case Sequential Reversed Data");
    sort_group.bench_function("insertion_sort_worst_case", |b| {
        b.iter(|| {
            insertion_sort(black_box(&mut reversed_array));
        });
    });

    sort_group.bench_function("insertion_sort_optimization_worst_case", |b| {
        b.iter(|| {
            insertion_sort_optimization(black_box(&mut reversed_array));
        });
    });

    sort_group.finish()
}

pub fn quick_sort_random_array(c: &mut Criterion) {
    let mut array: Vec<u32> = create_array(false, true);
    let mut quick_sort_group = c.benchmark_group("Quick Sort Algorithms Randomly Smaples");
    quick_sort_group.bench_function("Quick Sort with random pivot", |b| {
        b.iter(|| {
            quick_sort_random(black_box(&mut array));
        });
    });

    quick_sort_group.bench_function("Quick Sort with middle of three", |b| {
        b.iter(|| {
            quick_sort_middle_three(black_box(&mut array));
        });
    });

    quick_sort_group.bench_function("Quick Sort with last element", |b| {
        b.iter(|| {
            quick_sort_hoare(black_box(&mut array));
        });
    });

    quick_sort_group.finish()
}

pub fn insert_and_quick_sort(c: &mut Criterion) {
    let mut array: Vec<u32> = create_array(false, false);
    let mut sort_group = c.benchmark_group("Sort Algorithms Randomly Smaples");
    sort_group.bench_function("Insert Sort with ", |b| {
        b.iter(|| {
            insertion_sort_optimization(black_box(&mut array));
        });
    });

    sort_group.bench_function("merge_sort", |b| {
        b.iter(|| {
            quick_sort_with_custom_part(black_box(&mut array), &|size| {
                (0 + size.div_ceil(2) + size - 1).div_ceil(3)
            });
        });
    });

    sort_group.finish();
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = insertion_sort_average_case, insertion_sort_worst_case
);

criterion_main!(benches);
