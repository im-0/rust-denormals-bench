# rust-denormals-bench

This is a very simple benchmark that demonstrates the negative performance impact of subnormal (denormalized)
floating point values.

There are three benchmarks:

* **f32_normal** - `f32` (32-bit float) operations on normal values.
* **f32_subnormal** - same operations but on subnormal values.
* **f32_subnormal_flushed** - same operations on subnormal values but after enabling CPU-specific flush-to-zero mode.

The expectation is that on today's CPUs, **f32_normal** and **f32_subnormal_flushed** will be roughly the same, and
**f32_subnormal** will be slower.

## Usage

```bash
cargo run --release -- --bench
```

## Benchmark results

On AMD Ryzen 9 5950X: **x1.42 slower** <details><summary>click for details...</summary>

```text
f32_normal              time:   [737.13 ps 738.15 ps 739.04 ps]
                        change: [−0.4379% −0.1178% +0.2679%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) low severe
  3 (3.00%) low mild

f32_subnormal           time:   [1.0497 ns 1.0508 ns 1.0520 ns]
                        change: [−0.5409% −0.4133% −0.2375%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

f32_subnormal_flushed   time:   [729.36 ps 731.19 ps 733.08 ps]
                        change: [−0.0247% +0.2937% +0.6087%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

#rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8
```
<details>

On Intel Core i7-8565U: **x53.40 slower** <details><summary>click for details...</summary>

```text
f32_normal              time:   [842.54 ps 846.25 ps 853.96 ps]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

f32_subnormal           time:   [38.214 ns 38.237 ns 38.259 ns]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

f32_subnormal_flushed   time:   [806.57 ps 806.97 ps 807.38 ps]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  4 (4.00%) high severe

#rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8
```
<details>
