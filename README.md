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



Running benches/tlsh_build_bench.rs (target/release/deps/tlsh_build_bench-50750479aa4e76a9)
Timer precision: 20 ns

```
tlsh_build_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ build_fast_tlsh      17.06 ms      │ 22.49 ms      │ 17.17 ms      │ 17.49 ms      │ 100     │ 100
├─ build_simbiota_tlsh  65.99 ms      │ 80.73 ms      │ 66.55 ms      │ 67.04 ms      │ 100     │ 100
╰─ build_tlsh2          31.21 ms      │ 31.81 ms      │ 31.31 ms      │ 31.32 ms      │ 100     │ 100
```

Running benches/tlsh_diff_bench.rs (target/release/deps/tlsh_diff_bench-f393e696b6c3e0e7)
Timer precision: 20 ns

```
tlsh_diff_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ diff_fast_tlsh      4.459 ms      │ 5.219 ms      │ 4.539 ms      │ 4.552 ms      │ 100     │ 100
├─ diff_simbiota_tlsh  13.84 ms      │ 14.3 ms       │ 13.88 ms      │ 13.89 ms      │ 100     │ 100
╰─ diff_tlsh2          17.51 ms      │ 19.83 ms      │ 17.78 ms      │ 18 ms         │ 100     │ 100
```
