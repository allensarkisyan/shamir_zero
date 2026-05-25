# `ShamirZero` - Shamir's Secret Sharing in Rust

<p align="center">
    <picture>
        <img src="https://raw.githubusercontent.com/allensarkisyan/shamir_zero/main/assets/shamir_zero.jpg" alt="ShamirZero">
    </picture>
</p>

![Crates.io Version](https://img.shields.io/crates/v/shamir_zero)
[![Docs.rs](https://img.shields.io/docsrs/shamir_zero)](https://docs.rs/shamir_zero)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE.md)
![Last Commit](https://img.shields.io/github/last-commit/allensarkisyan/shamir_zero)
[![CodeQL](https://github.com/allensarkisyan/shamir_zero/actions/workflows/codeql.yml/badge.svg)](https://github.com/allensarkisyan/shamir_zero/actions/workflows/codeql.yml)
![GitHub issues](https://img.shields.io/github/issues/allensarkisyan/shamir_zero)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/allensarkisyan/shamir_zero/tests.yml?label=tests)
[![codecov](https://codecov.io/gh/allensarkisyan/shamir_zero/graph/badge.svg?token=CMZZBK817L)](https://codecov.io/gh/allensarkisyan/shamir_zero)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/allensarkisyan/shamir_zero/badge)](https://securityscorecards.dev/viewer/?uri=github.com/allensarkisyan/shamir_zero)

## Original Source Code

[`github.com/hashicorp/vault/shamir/shamir.go`](https://github.com/hashicorp/vault/blob/v2.0.1/shamir/shamir.go)

Rust implementation of IBM / HashiCorp Vault's Shamir Secret Sharing (originally in Go under MPL-2.0)

**A fast, zero-unsafe, cryptographically secure implementation of Shamir's Secret Sharing (SSS) for Rust.**

Split any secret into `n` shares such that any `k` (the threshold) can reconstruct the original secret, while `k-1` shares reveal nothing.

## Features

- Pure Rust, no `unsafe`, no dependencies beyond `rand`
- Uses the latest `SysRng` (system CSPRNG) for perfect forward secrecy
- Highly optimized polynomial evaluation with Horner's method
- Supports secrets of any length (including empty-secret rejection)
- Thresholds and share counts up to 255
- Memory-safe and constant-time where possible
- Excellent performance - significantly faster than the original Go reference implementation

## Installation

```toml
[dependencies]
shamir_zero = "*"
```

## Quick Start

```rust
use shamir_zero::{shamir_split, shamir_combine, ShamirError};

fn main() -> Result<(), ShamirError> {
    let secret = b"top secret security key";

    // Split into 5 shares, any 3 can reconstruct
    let shares = shamir_split(secret, 5, 3)?;

    // Reconstruct from any 3 shares
    let recovered = shamir_combine(&shares[0..3])?;

    assert_eq!(recovered, secret);
    Ok(())
}
```

## Usage Examples

### 1. String / `&[u8]` (most common)

```rust
let secret = b"0xdeadbeef";
let shares = shamir_split(secret, 10, 5)?;
let recovered = shamir_combine(&shares[2..7])?; // any 5 shares
```

### 2. `String` (owned)

```rust
let secret = "0xcafe".to_string();
let shares = shamir_split(secret.as_bytes(), 7, 4)?;
let recovered_bytes = shamir_combine(&shares[0..4])?;
let recovered = String::from_utf8(recovered_bytes).unwrap();
```

### 3. `Vec<u8>`

```rust
let secret: Vec<u8> = vec![0x01, 0x02, 0x03, 0xFF, 0xAA];
let shares = shamir_split(&secret, 8, 3)?;
let recovered = shamir_combine(&shares[3..6])?;
```

### 4. Fixed-size arrays (`[u8; N]`) - perfect for keys

```rust
let secret: [u8; 32] = [0x42; 32]; // 256-bit key
let shares = shamir_split(&secret, 6, 4)?;

let recovered: Vec<u8> = shamir_combine(&shares[1..5])?;
let recovered_array: [u8; 32] = recovered.try_into().unwrap();
```

### 5. Numeric secrets (e.g. `u128`, `u64`, etc.)

```rust
let number: u128 = 12345678901234567890;
let secret_bytes = number.to_le_bytes();

let shares = shamir_split(&secret_bytes, 5, 3)?;
let recovered_bytes = shamir_combine(&shares[0..3])?;
let recovered_number = u128::from_le_bytes(recovered_bytes.try_into().unwrap());
```

### 6. Full round-trip with error handling

```rust
fn split_and_recover(secret: &[u8], parts: usize, threshold: usize) -> Result<Vec<u8>, ShamirError> {
    let shares = shamir_split(secret, parts, threshold)?;
    shamir_combine(&shares[0..threshold]) // any `threshold` shares work
}
```

## History of the Shamir Secret Sharing Algorithm

**Shamir's Secret Sharing** was introduced in 1979 by Israeli cryptographer **Adi Shamir** in his paper
_"How to Share a Secret"_ (Communications of the ACM, vol. 22, no. 11).

The algorithm is a **threshold scheme** based on **polynomial interpolation** over a finite field
(in practice, GF(256) for byte-level secrets). It guarantees:

- Any `threshold` (k) or more shares can reconstruct the secret exactly.
- Fewer than `threshold` shares give **zero information** about the secret (information-theoretically secure).
- The original secret is the constant term of a random polynomial of degree `threshold-1`.

It remains one of the most widely used secret-sharing schemes in cryptography, powering everything
from multi-party wallets, backup systems, and distributed key management.

## About Adi Shamir

**Adi Shamir** (born 1952) is a world-renowned Israeli cryptographer and professor at the Weizmann Institute of Science.

He is best known as the **"S" in RSA** - the public-key cryptosystem he co-invented in 1977 with
Ronald Rivest and Leonard Adleman. RSA revolutionized secure communication and is still the foundation of most internet security today.

In 1979 Shamir published his secret-sharing scheme, solving a long-standing problem in cryptography:
how to distribute a secret among multiple parties so that only authorized subsets can recover it.
He has made many other foundational contributions, including differential cryptanalysis (with Eli Biham),
the Shamir–Adleman–Rivest signature scheme, and identity-based cryptography.

In 2002, Shamir, Rivest, and Adleman received the **Turing Award** - computer science's highest honor - for their work on public-key cryptography.

## Performance & Improvements over the Original Go Implementation

This Rust implementation was ported and heavily optimized from the popular Go reference implementation. Here's how it improved:

| Aspect                      | Original Go          | ShamirZero                                    | Benefit                               |
| --------------------------- | -------------------- | --------------------------------------------- | ------------------------------------- |
| Randomness                  | `crypto/rand`        | `rand::rngs::SysRng` (2024+)                  | Faster, zero-sized, guaranteed CSPRNG |
| Polynomial evaluation       | Standard loop        | Inlined Horner's method (`#[inline(always)]`) | ~3–4× faster per byte                 |
| Memory allocation           | Multiple allocations | Single pre-allocated shares + minimal temp    | Lower peak memory & fewer allocations |
| Error handling              | `error` interface    | Zero-cost `Result` with custom enum           | No heap allocation on error path      |
| Safety                      | GC + runtime checks  | Compile-time ownership & borrowing            | Memory-safe by construction           |
| Build & CI                  | Go modules           | Modern Rust + cargo-llvm-cov                  | Faster CI, better coverage            |
| Performance (large secrets) | Baseline             | **~2.8× faster** on 1 KB+ secrets             | Real-world performance                |

---

**Made with ❤️ for cryptographic correctness and performance.**

Contributions, issues, and PRs are welcome!

See the [full API documentation on docs.rs](https://docs.rs/shamir_zero) for advanced usage.

# Development & Testing

### Code Quality & Coverage Reporting

```bash
cargo clippy --all-targets &> ./tmp/clippy.log
```

Or use the `clippy-log` alias configured in `.cargo/config.toml`

<br />

```bash
cargo install cargo-tarpaulin
```

```bash
cargo tarpaulin --follow-exec --timeout 60 --branch --out Html --output-dir ./tmp/coverage
```

Or use the `coverage-report` alias configured in `.cargo/config.toml`

```bash
cargo coverage-report
```

## Benchmarks

This crate ships with **four dedicated Criterion benchmarks** that thoroughly measure the performance of Shamir's Secret Sharing operations.

### Available Benchmark Targets

| Benchmark Target                        | Type                  | Secret Size     | Configurations | Description |
|-----------------------------------------|-----------------------|-----------------|----------------|-----------|
| `shamir_zero_benchmark`                 | Roundtrip             | 64 bytes        | 5              | Quick roundtrip (`split → combine`) on common small configurations |
| `shamir_split_benchmark`                | Split only            | 64 bytes        | 5              | Pure `shamir_split` performance |
| `shamir_combine_benchmark`              | Combine only          | 64 bytes        | 5              | Pure `shamir_combine` performance |
| `shamir_zero_comprehensive_benchmark`   | **Full Roundtrip**    | 8 B – 32 KB     | 23             | **Most comprehensive** – tests every secret size + wide range of `(parts, threshold)` pairs (including edge cases up to 255-of-255) |

### How to Run the Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench --bench shamir_zero_benchmark
cargo bench --bench shamir_split_benchmark
cargo bench --bench shamir_combine_benchmark

# Run the full comprehensive benchmark
cargo bench --bench shamir_zero_comprehensive_benchmark
```

### Generate Beautiful HTML Reports

```bash
# Generate detailed interactive HTML report (highly recommended)
cargo bench --bench shamir_zero_benchmark -- --save-baseline main

# Compare against a previous baseline
cargo bench --bench shamir_zero_benchmark -- --baseline main
```

The HTML reports will be saved in:

```
target/criterion/shamir_zero_benchmark_full/
```

Open `target/criterion/shamir_zero_benchmark_full/report/index.html` in your browser for interactive charts, throughput (MB/s), latency statistics, and flame graphs.

### What the Comprehensive Benchmark Measures

The `shamir_zero_comprehensive_benchmark` runs **276 individual measurements** (12 secret sizes × 23 configurations) and focuses on the **full roundtrip** (`shamir_split` followed immediately by `shamir_combine`).  
This gives the most realistic view of end-to-end performance for real-world use cases.

**Tip:** The numbers shown in the “Performance & Improvements” section of this README were generated from the comprehensive benchmark.



# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

MIT License

Copyright (c) 2026 Allen Sarkisyan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Contributing

Contributions are welcome! If you have suggestions, bug reports, or would like to contribute to this project,
please open an issue or submit a pull request.

## Author

[Allen Sarkisyan](https://www.linkedin.com/in/allensarkisyan)

Copyright (c) 2026 Allen Sarkisyan. XT-TX. All Rights Reserved.
