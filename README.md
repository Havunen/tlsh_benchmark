# TLSH Benchmark

This repository compares different TLSH algorithm implementations in Rust.
Benchmark is run using 128bit - 1byte checksum hashes.

There are two benchmarks:
- `tlsh_construct_bench` - compares the performance of creating TLSH struct from data row
- `tlsh_diff_bench` - compares the performance of calculating the difference of TLSH hashes

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
├─ construct_fast_tlsh      16.78 ms      │ 42.61 ms      │ 16.88 ms      │ 17.81 ms      │ 100     │ 100
├─ construct_simbiota_tlsh  64.55 ms      │ 70.86 ms      │ 65.36 ms      │ 65.77 ms      │ 100     │ 100
├─ construct_tlsh2          31.24 ms      │ 34.42 ms      │ 31.45 ms      │ 31.64 ms      │ 100     │ 100
╰─ construct_tlsh_orig      55.82 ms      │ 61.18 ms      │ 56.18 ms      │ 56.37 ms      │ 100     │ 100
```

Running benches/tlsh_diff_bench.rs (target/release/deps/tlsh_diff_bench-f393e696b6c3e0e7)
Timer precision: 20 ns

```
tlsh_diff_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ diff_fast_tlsh      4.492 ms      │ 5.29 ms       │ 4.58 ms       │ 4.591 ms      │ 100     │ 100
├─ diff_simbiota_tlsh  13.88 ms      │ 14.72 ms      │ 14.01 ms      │ 14.03 ms      │ 100     │ 100
├─ diff_tlsh2          17.52 ms      │ 20.72 ms      │ 17.93 ms      │ 18.06 ms      │ 100     │ 100
╰─ diff_tlsh_orig      23.43 ms      │ 41.19 ms      │ 23.8 ms       │ 24.08 ms      │ 100     │ 100

```
