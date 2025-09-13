// SPDX-License-Identifier: Apache-2.0

use std::env::args;
use std::process::{Command, exit};

use criterion::{Criterion, criterion_group};
use rand::random_range;

mod flush;

fn main() {
    if args().len() < 2 {
        let status = Command::new(args().next().expect("No program path in argv"))
            .arg("--bench")
            .status()
            .expect("Failed to execute self");
        exit(status.code().expect("No exit code"));
    }

    benches();
    Criterion::default().configure_from_args().final_summary();
}

criterion_group!(
    benches,
    bench_f32_normal,
    bench_f32_subnormal,
    bench_f32_subnormal_flushed,
);

fn bench_f32_normal(criterion: &mut Criterion) {
    let (v1, v2, v3, v4, v5, v6) = get_normal_values();

    criterion.bench_function("f32_normal", |b| {
        b.iter(|| fp_op(v1, v2, v3, v4, v5, v6));
    });
}

fn bench_f32_subnormal(criterion: &mut Criterion) {
    let (v1, v2, v3, v4, v5, v6) = get_subnormal_values();

    criterion.bench_function("f32_subnormal", |b| {
        b.iter(|| fp_op(v1, v2, v3, v4, v5, v6));
    });
}

fn bench_f32_subnormal_flushed(criterion: &mut Criterion) {
    let (v1, v2, v3, v4, v5, v6) = get_subnormal_values();

    criterion.bench_function("f32_subnormal_flushed", |b| {
        flush::enable_flush_to_zero!();
        b.iter(|| fp_op(v1, v2, v3, v4, v5, v6));
    });
}

fn get_normal_values() -> (f32, f32, f32, f32, f32, f32) {
    let v1 = random_range(1..=2) as f32;
    let v2 = random_range(1..=2) as f32;
    let v3 = random_range(1..=2) as f32;
    let v4 = random_range(1..=2) as f32;
    let v5 = random_range(1..=2) as f32;
    let v6 = random_range(1..=2) as f32;

    assert!(v1.is_normal());
    assert!(v2.is_normal());
    assert!(v3.is_normal());
    assert!(v4.is_normal());
    assert!(v5.is_normal());
    assert!(v6.is_normal());

    (v1, v2, v3, v4, v5, v6)
}

fn get_subnormal_values() -> (f32, f32, f32, f32, f32, f32) {
    let v1 = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };
    let v2 = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };
    let v3 = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };
    let v4 = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };
    let v5 = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };
    let v6 = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };

    assert!(v1.is_subnormal());
    assert!(v2.is_subnormal());
    assert!(v3.is_subnormal());
    assert!(v4.is_subnormal());
    assert!(v5.is_subnormal());
    assert!(v6.is_subnormal());

    (v1, v2, v3, v4, v5, v6)
}

#[inline(always)]
#[must_use]
fn fp_op(v1: f32, v2: f32, v3: f32, v4: f32, v5: f32, v6: f32) -> f32 {
    v1 / v2 + v3 * v4 * v5 * v6
}
