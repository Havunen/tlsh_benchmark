# TLSH Benchmark

This repository compares different TLSH algorithm implementations in Rust.
Benchmark is run using 128bit - 1byte checksum hashes.

## Specification

For more information about TLSH goto: https://tlsh.org/

## Results

Used system:
```bash
Arch Linux 6.14.2-arch1-1 #1 SMP PREEMPT_DYNAMIC Thu, 10 Apr 2025 18:43:59 +0000 x86_64 GNU/Linux
AMD Ryzen 9 5950X 16-Core Processor
128GB RAM 3600Mhz
```



Running benches/tlsh_construct_bench.rs (target/release/deps/tlsh_construct_bench-50750479aa4e76a9)
Timer precision: 20 ns

```
tlsh_construct_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ construct_fast_tlsh      16.97 ms      │ 18.11 ms      │ 17.13 ms      │ 17.15 ms      │ 100     │ 100
├─ construct_simbiota_tlsh  65.15 ms      │ 68.61 ms      │ 66.04 ms      │ 66.09 ms      │ 100     │ 100
╰─ construct_tlsh2          31.19 ms      │ 36.85 ms      │ 31.47 ms      │ 31.73 ms      │ 100     │ 100
```

Running benches/tlsh_diff_bench.rs (target/release/deps/tlsh_diff_bench-f393e696b6c3e0e7)
Timer precision: 20 ns

```
tlsh_diff_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ diff_fast_tlsh      4.487 ms      │ 6.022 ms      │ 4.57 ms       │ 4.601 ms      │ 100     │ 100
├─ diff_simbiota_tlsh  13.95 ms      │ 14.39 ms      │ 13.96 ms      │ 13.97 ms      │ 100     │ 100
╰─ diff_tlsh2          16.99 ms      │ 17.58 ms      │ 17.41 ms      │ 17.4 ms       │ 100     │ 100
```
