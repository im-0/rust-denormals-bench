// SPDX-License-Identifier: Apache-2.0

use std::env::args;
use std::hint::black_box;
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
    let v1 = rand_normal_f32();
    let v2 = rand_normal_f32();
    let v3 = rand_normal_f32();
    let v4 = rand_normal_f32();
    let v5 = rand_normal_f32();
    let v6 = rand_normal_f32();

    criterion.bench_function("f32_normal", |b| {
        b.iter(|| fp_op(v1, v2, v3, v4, v5, v6));
    });
}

fn bench_f32_subnormal(criterion: &mut Criterion) {
    let v1 = rand_subnormal_f32();
    let v2 = rand_subnormal_f32();
    let v3 = rand_subnormal_f32();
    let v4 = rand_subnormal_f32();
    let v5 = rand_subnormal_f32();
    let v6 = rand_subnormal_f32();

    criterion.bench_function("f32_subnormal", |b| {
        b.iter(|| fp_op(v1, v2, v3, v4, v5, v6));
    });
}

fn bench_f32_subnormal_flushed(criterion: &mut Criterion) {
    let v1 = rand_subnormal_f32();
    let v2 = rand_subnormal_f32();
    let v3 = rand_subnormal_f32();
    let v4 = rand_subnormal_f32();
    let v5 = rand_subnormal_f32();
    let v6 = rand_subnormal_f32();

    criterion.bench_function("f32_subnormal_flushed", |b| {
        flush::enable_flush_to_zero!();
        b.iter(|| fp_op(v1, v2, v3, v4, v5, v6));
    });
}

#[inline(always)]
#[must_use]
fn rand_normal_f32() -> f32 {
    let v = random_range(1..=2) as f32;
    assert!(v.is_normal());
    v
}

#[inline(always)]
#[must_use]
fn rand_subnormal_f32() -> f32 {
    let v = if random_range(1..=2) == 1 {
        0.0f32.next_up()
    } else {
        0.0f32.next_up().next_up()
    };
    assert!(v.is_subnormal());
    v
}

#[inline(always)]
#[must_use]
fn fp_op(v1: f32, v2: f32, v3: f32, v4: f32, v5: f32, v6: f32) -> f32 {
    black_box(v1) / black_box(v2) + black_box(v3) * black_box(v4) * black_box(v5) * black_box(v6)
}
