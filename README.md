# Similarity hash Benchmark

This repository was originally created to benchmark the performance of different implementations of TLSH (Trend Locality Sensitive Hashing) algorithms. TLSH is a locality-sensitive hashing algorithm that is used to compare the similarity of data, such as files or strings.
Later, it was extended to include other similarity hash algorithms.

There are two benchmarks:
- `tlsh_construct_bench` - compares the performance of creating TLSH struct from data row
- `tlsh_diff_bench` - compares the performance of calculating the difference of TLSH hashes

## Specification

For more information about TLSH goto: https://tlsh.org/

## Results

Used system:
```bash
NixOS Linux 6.15.7 #1-NixOS SMP PREEMPT_DYNAMIC Thu Jul 17 16:44:05 UTC 2025 x86_64 GNU/Linux
Intel Core Ultra 7 155H 22-Core Processor
32GB RAM 7467Mhz
```



Running benches/tlsh_construct_bench.rs (target/release/deps/tlsh_construct_bench-c1c03f39143dd24a)
Timer precision: 10 ns

```
tlsh_construct_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ construct_fast_tlsh      16.2 ms       │ 16.71 ms      │ 16.29 ms      │ 16.3 ms       │ 100     │ 100
├─ construct_ffuzzy         37.63 ms      │ 38.39 ms      │ 37.79 ms      │ 37.81 ms      │ 100     │ 100
├─ construct_simbiota_tlsh  42.76 ms      │ 43.39 ms      │ 43.02 ms      │ 43.02 ms      │ 100     │ 100
├─ construct_tlsh2          19.38 ms      │ 19.48 ms      │ 19.43 ms      │ 19.43 ms      │ 100     │ 100
╰─ construct_tlsh_orig      49.12 ms      │ 49.39 ms      │ 49.25 ms      │ 49.25 ms      │ 100     │ 100

```

Running benches/tlsh_diff_bench.rs (target/release/deps/tlsh_diff_bench-ed6bf55555eeaf47)
Timer precision: 10 ns

```
tlsh_diff_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ diff_fast_tlsh      4.593 ms      │ 5.878 ms      │ 4.68 ms       │ 4.709 ms      │ 100     │ 100
├─ diff_ffuzzy         53.64 ms      │ 55.04 ms      │ 53.77 ms      │ 53.79 ms      │ 100     │ 100
├─ diff_simbiota_tlsh  203.5 ms      │ 204.7 ms      │ 203.6 ms      │ 203.7 ms      │ 100     │ 100
├─ diff_tlsh2          15.43 ms      │ 16.65 ms      │ 15.52 ms      │ 15.56 ms      │ 100     │ 100
╰─ diff_tlsh_orig      25.46 ms      │ 26.5 ms       │ 25.71 ms      │ 25.8 ms       │ 100     │ 100

```
