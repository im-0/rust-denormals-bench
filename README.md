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
</details>

On Intel Core i9-9900K: **x50.31 slower** <details><summary>click for details...</summary>

```text
f32_normal              time:   [697.17 ps 697.95 ps 698.83 ps]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

f32_subnormal           time:   [35.085 ns 35.114 ns 35.148 ns]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

f32_subnormal_flushed   time:   [686.81 ps 688.20 ps 690.18 ps]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

# rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc41)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: x86_64-unknown-linux-gnu
release: 1.89.0
LLVM version: 19.1.7
```
</details>

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
</details>

On Apple M2 Max: **no slowdown** <details><summary>click for details...</summary>

```text
f32_normal              time:   [654.07 ps 654.11 ps 654.15 ps]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe

f32_subnormal           time:   [653.40 ps 653.46 ps 653.53 ps]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe

f32_subnormal_flushed   time:   [653.34 ps 653.41 ps 653.51 ps]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

#rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: aarch64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8
```
</details>

On Broadcom BCM2711 (Raspberry Pi 4): **no slowdown** <details><summary>click for details...</summary>

```text
f32_normal              time:   [3.3524 ns 3.3537 ns 3.3553 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

f32_subnormal           time:   [3.3511 ns 3.3515 ns 3.3520 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

f32_subnormal_flushed   time:   [3.3532 ns 3.3537 ns 3.3542 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

#rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: aarch64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8
```
</details>

On Qualcomm Snapdragon SC8280XP: **no slowdown** <details><summary>click for details...</summary>

```text
f32_normal              time:   [786.49 ps 786.74 ps 787.02 ps]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

f32_subnormal           time:   [787.63 ps 787.77 ps 787.96 ps]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) low severe
  3 (3.00%) high mild
  3 (3.00%) high severe

f32_subnormal_flushed   time:   [786.92 ps 786.97 ps 787.03 ps]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe

#rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: aarch64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8
```
</details>

On Broadcom BCM2710A1 (Raspberry Pi Zero 2 W): **x1.04 slower** <details><summary>click for details...</summary>

```text
f32_normal              time:   [22.182 ns 22.195 ns 22.213 ns]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

f32_subnormal           time:   [23.187 ns 23.194 ns 23.203 ns]
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

f32_subnormal_flushed   time:   [22.205 ns 22.225 ns 22.248 ns]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

#rustc -vV
rustc 1.89.0 (29483883e 2025-08-04) (Fedora 1.89.0-2.fc42)
binary: rustc
commit-hash: 29483883eed69d5fb4db01964cdf2af4d86e9cb2
commit-date: 2025-08-04
host: aarch64-unknown-linux-gnu
release: 1.89.0
LLVM version: 20.1.8
```
</details>
