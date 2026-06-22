## `32 bit`

```log
    Finished `bench` profile [optimized] target(s) in 0.05s
     Running benches/roundtrip.rs (target/release/deps/roundtrip-11cfb73a7b22af35)
Gnuplot not found, using plotters backend
Benchmarking shamir_roundtrip/shamir-zero/32B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero/32B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero/32B_t5_n10: Collecting 500 samples in estimated 8.4594 s (1.1M iterations)
Benchmarking shamir_roundtrip/shamir-zero/32B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero/32B_t5_n10
                        time:   [7.5287 µs 7.5330 µs 7.5373 µs]
Found 21 outliers among 500 measurements (4.20%)
  6 (1.20%) low mild
  10 (2.00%) high mild
  5 (1.00%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/32B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/32B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/32B_t5_n10: Collecting 500 samples in estimated 8.5307 s (1.1M iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/32B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy/32B_t5_n10
                        time:   [7.5497 µs 7.5541 µs 7.5586 µs]
Found 13 outliers among 500 measurements (2.60%)
  6 (1.20%) low mild
  5 (1.00%) high mild
  2 (0.40%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/32B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/32B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/32B_t5_n10: Collecting 500 samples in estimated 8.2815 s (1.1M iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/32B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy-allocated/32B_t5_n10
                        time:   [7.3121 µs 7.3162 µs 7.3203 µs]
Found 18 outliers among 500 measurements (3.60%)
  1 (0.20%) low mild
  14 (2.80%) high mild
  3 (0.60%) high severe
```

## `64 bit`

```log
Benchmarking shamir_roundtrip/shamir-zero/64B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero/64B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero/64B_t5_n10: Collecting 500 samples in estimated 8.9074 s (626k iterations)
Benchmarking shamir_roundtrip/shamir-zero/64B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero/64B_t5_n10
                        time:   [14.188 µs 14.195 µs 14.202 µs]
Found 19 outliers among 500 measurements (3.80%)
  1 (0.20%) low mild
  10 (2.00%) high mild
  8 (1.60%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/64B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/64B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/64B_t5_n10: Collecting 500 samples in estimated 8.9411 s (626k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/64B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy/64B_t5_n10
                        time:   [14.204 µs 14.212 µs 14.221 µs]
Found 15 outliers among 500 measurements (3.00%)
  9 (1.80%) low mild
  5 (1.00%) high mild
  1 (0.20%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/64B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/64B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/64B_t5_n10: Collecting 500 samples in estimated 8.7738 s (626k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/64B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy-allocated/64B_t5_n10
                        time:   [13.974 µs 13.983 µs 13.994 µs]
Found 18 outliers among 500 measurements (3.60%)
  9 (1.80%) low mild
  7 (1.40%) high mild
  2 (0.40%) high severe
```

## `128 bit`

```log
   Compiling shamir-bench v0.1.0 (/home/xttx/dev/github/shamir-rs/examples/benchmark)
    Finished `bench` profile [optimized] target(s) in 1.22s
     Running benches/roundtrip.rs (target/release/deps/roundtrip-11cfb73a7b22af35)
Gnuplot not found, using plotters backend
Benchmarking shamir_roundtrip/shamir-zero/128B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero/128B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero/128B_t5_n10: Collecting 500 samples in estimated 10.415 s (376k iterations)
Benchmarking shamir_roundtrip/shamir-zero/128B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero/128B_t5_n10
                        time:   [27.502 µs 27.530 µs 27.564 µs]
Found 15 outliers among 500 measurements (3.00%)
  10 (2.00%) high mild
  5 (1.00%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/128B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/128B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/128B_t5_n10: Collecting 500 samples in estimated 10.393 s (376k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/128B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy/128B_t5_n10
                        time:   [27.556 µs 27.571 µs 27.586 µs]
Found 19 outliers among 500 measurements (3.80%)
  5 (1.00%) low mild
  10 (2.00%) high mild
  4 (0.80%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/128B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/128B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/128B_t5_n10: Collecting 500 samples in estimated 10.270 s (376k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/128B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy-allocated/128B_t5_n10
                        time:   [27.293 µs 27.313 µs 27.338 µs]
Found 9 outliers among 500 measurements (1.80%)
  6 (1.20%) high mild
  3 (0.60%) high severe
```


## `256 bit`



```log
Benchmarking shamir_roundtrip/shamir-zero/256B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero/256B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero/256B_t5_n10: Collecting 500 samples in estimated 13.585 s (250k iterations)
Benchmarking shamir_roundtrip/shamir-zero/256B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero/256B_t5_n10
                        time:   [54.031 µs 54.059 µs 54.087 µs]
Found 8 outliers among 500 measurements (1.60%)
  1 (0.20%) low mild
  4 (0.80%) high mild
  3 (0.60%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/256B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/256B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/256B_t5_n10: Collecting 500 samples in estimated 13.629 s (250k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/256B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy/256B_t5_n10
                        time:   [54.093 µs 54.123 µs 54.153 µs]
Found 15 outliers among 500 measurements (3.00%)
  1 (0.20%) low mild
  10 (2.00%) high mild
  4 (0.80%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/256B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/256B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/256B_t5_n10: Collecting 500 samples in estimated 13.499 s (250k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/256B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy-allocated/256B_t5_n10
                        time:   [53.807 µs 53.833 µs 53.860 µs]
Found 12 outliers among 500 measurements (2.40%)
  9 (1.80%) high mild
  3 (0.60%) high severe
```

## `1024 bit`

```log
   Compiling shamir-bench v0.1.0 (/home/xttx/dev/github/shamir-rs/examples/benchmark)
    Finished `bench` profile [optimized] target(s) in 1.16s
     Running benches/roundtrip.rs (target/release/deps/roundtrip-11cfb73a7b22af35)
Gnuplot not found, using plotters backend
Benchmarking shamir_roundtrip/shamir-zero/1024B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero/1024B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero/1024B_t5_n10: Collecting 500 samples in estimated 8.0262 s (38k iterations)
Benchmarking shamir_roundtrip/shamir-zero/1024B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero/1024B_t5_n10
                        time:   [213.33 µs 213.44 µs 213.55 µs]
Found 5 outliers among 500 measurements (1.00%)
  4 (0.80%) high mild
  1 (0.20%) high severe
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/1024B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/1024B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/1024B_t5_n10: Collecting 500 samples in estimated 8.0274 s (38k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/1024B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy/1024B_t5_n10
                        time:   [213.61 µs 213.71 µs 213.81 µs]
Found 6 outliers among 500 measurements (1.20%)
  6 (1.20%) high mild
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/1024B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/1024B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/1024B_t5_n10: Collecting 500 samples in estimated 8.0118 s (38k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/1024B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy-allocated/1024B_t5_n10
                        time:   [213.00 µs 213.10 µs 213.19 µs]
Found 6 outliers among 500 measurements (1.20%)
  6 (1.20%) high mild
```


## `2048 bit`

```log
Benchmarking shamir_roundtrip/shamir-zero/2048B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero/2048B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero/2048B_t5_n10: Collecting 500 samples in estimated 8.1305 s (19k iterations)
Benchmarking shamir_roundtrip/shamir-zero/2048B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero/2048B_t5_n10
                        time:   [426.96 µs 427.21 µs 427.46 µs]
Found 6 outliers among 500 measurements (1.20%)
  6 (1.20%) high mild
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/2048B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/2048B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/2048B_t5_n10: Collecting 500 samples in estimated 8.1416 s (19k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy/2048B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy/2048B_t5_n10
                        time:   [426.18 µs 426.38 µs 426.58 µs]
Found 2 outliers among 500 measurements (0.40%)
  2 (0.40%) high mild
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/2048B_t5_n10
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/2048B_t5_n10: Warming up for 3.0000 s
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/2048B_t5_n10: Collecting 500 samples in estimated 8.1017 s (19k iterations)
Benchmarking shamir_roundtrip/shamir-zero-zero-copy-allocated/2048B_t5_n10: Analyzing
shamir_roundtrip/shamir-zero-zero-copy-allocated/2048B_t5_n10
                        time:   [425.87 µs 426.23 µs 426.67 µs]
Found 2 outliers among 500 measurements (0.40%)
  2 (0.40%) high severe
```