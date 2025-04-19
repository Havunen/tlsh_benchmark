# TLSH Benchmark

This repository compares different TLSH algorithm implementations in Rust.
Benchmark is run using 128bit - 1byte checksum hashes.

## Specification

For more information about TLSH goto: https://tlsh.org/

## Results

Used system:
```
Arch Linux 6.14.2-arch1-1 #1 SMP PREEMPT_DYNAMIC Thu, 10 Apr 2025 18:43:59 +0000 x86_64 GNU/Linux
AMD Ryzen 9 5950X 16-Core Processor
128GB RAM 3600Mhz
```



     Running benches/tlsh_build_bench.rs (target/release/deps/tlsh_build_bench-50750479aa4e76a9)
Timer precision: 20 ns
tlsh_build_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ build_fast_tlsh      17.02 ms      │ 20.34 ms      │ 17.18 ms      │ 17.33 ms      │ 100     │ 100
│                       max alloc:    │               │               │               │         │
│                         1           │ 1             │ 1             │ 1             │         │
│                         480 KB      │ 480 KB        │ 480 KB        │ 480 KB        │         │
│                       alloc:        │               │               │               │         │
│                         1           │ 1             │ 1             │ 1             │         │
│                         480 KB      │ 480 KB        │ 480 KB        │ 480 KB        │         │
│                       dealloc:      │               │               │               │         │
│                         1           │ 1             │ 1             │ 1             │         │
│                         480 KB      │ 480 KB        │ 480 KB        │ 480 KB        │         │
├─ build_simbiota_tlsh  64.05 ms      │ 65.82 ms      │ 64.72 ms      │ 64.77 ms      │ 100     │ 100
│                       max alloc:    │               │               │               │         │
│                         10002       │ 10002         │ 10002         │ 10002         │         │
│                         941.3 KB    │ 941.3 KB      │ 941.3 KB      │ 941.3 KB      │         │
│                       alloc:        │               │               │               │         │
│                         30001       │ 30001         │ 30001         │ 30001         │         │
│                         14.55 MB    │ 14.55 MB      │ 14.55 MB      │ 14.55 MB      │         │
│                       dealloc:      │               │               │               │         │
│                         30001       │ 30001         │ 30001         │ 30001         │         │
│                         14.55 MB    │ 14.55 MB      │ 14.55 MB      │ 14.55 MB      │         │
╰─ build_tlsh2          31.2 ms       │ 33.56 ms      │ 31.42 ms      │ 31.45 ms      │ 100     │ 100
                        max alloc:    │               │               │               │         │
                          1           │ 1             │ 1             │ 1             │         │
                          720 KB      │ 720 KB        │ 720 KB        │ 720 KB        │         │
                        alloc:        │               │               │               │         │
                          1           │ 1             │ 1             │ 1             │         │
                          720 KB      │ 720 KB        │ 720 KB        │ 720 KB        │         │
                        dealloc:      │               │               │               │         │
                          1           │ 1             │ 1             │ 1             │         │
                          720 KB      │ 720 KB        │ 720 KB        │ 720 KB        │         │

     Running benches/tlsh_diff_bench.rs (target/release/deps/tlsh_diff_bench-f393e696b6c3e0e7)
Timer precision: 20 ns
tlsh_diff_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ diff_fast_tlsh      4.341 ms      │ 5.767 ms      │ 4.451 ms      │ 4.489 ms      │ 100     │ 100
│                      max alloc:    │               │               │               │         │
│                        1           │ 1             │ 1             │ 1             │         │
│                        4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
│                      alloc:        │               │               │               │         │
│                        1           │ 1             │ 1             │ 1             │         │
│                        4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
│                      dealloc:      │               │               │               │         │
│                        1           │ 1             │ 1             │ 1             │         │
│                        4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
├─ diff_simbiota_tlsh  13.9 ms       │ 15.8 ms       │ 13.96 ms      │ 14.05 ms      │ 100     │ 100
│                      max alloc:    │               │               │               │         │
│                        1           │ 1             │ 1             │ 1             │         │
│                        4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
│                      alloc:        │               │               │               │         │
│                        1           │ 1             │ 1             │ 1             │         │
│                        4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
│                      dealloc:      │               │               │               │         │
│                        1           │ 1             │ 1             │ 1             │         │
│                        4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
╰─ diff_tlsh2          17.36 ms      │ 17.9 ms       │ 17.79 ms      │ 17.73 ms      │ 100     │ 100
                       max alloc:    │               │               │               │         │
                         1           │ 1             │ 1             │ 1             │         │
                         4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
                       alloc:        │               │               │               │         │
                         1           │ 1             │ 1             │ 1             │         │
                         4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │
                       dealloc:      │               │               │               │         │
                         1           │ 1             │ 1             │ 1             │         │
                         4 MB        │ 4 MB          │ 4 MB          │ 4 MB          │         │

