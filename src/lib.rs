// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (C) 2026 Allen Sarkisyan

//! # `shamir-zero` - Shamir's Secret Sharing in Rust
//!
//! A fast, zero-unsafe, cryptographically secure implementation of Shamir's Secret Sharing (SSS) for Rust.
//! Split any secret into `n` shares such that any `k` (the threshold) can reconstruct the original secret,
//! while `k-1` shares reveal nothing.
//!
//! ## Key Features
//!
//! - **True Zero-Copy Core API**: `shamir_split` accepts a pre-allocated output buffer to eliminate intermediate allocations and unpredictable memory overhead.
//! - **High-Level Convenience Wrapper**: `ShamirZero` provides a familiar `Vec<Vec<u8>>` interface while internally allocating exactly once and delegating to the optimized core.
//! - **Fast Inverse (Default)**: Uses a compile-time 256-byte lookup table for multiplicative inversion in GF(2^8). Dramatically faster and still fully constant-time.
//! - **Pure Rust & Safe**: No `unsafe` code, no dependencies beyond `rand`.
//! - **Cryptographically Secure**: Uses `rand::rngs::SysRng` (system CSPRNG) for perfect forward secrecy.
//! - **Threshold Support**: Supports thresholds and share counts up to 255.
//!
//! ## Quick Start
//!
//! ```
//! use shamir_zero::{ShamirZero, ShamirError};
//!
//! let secret = b"top secret security key";
//! let shares = ShamirZero::split(secret, 5, 3).unwrap();
//! let recovered = ShamirZero::combine(&shares[0..3]).unwrap();
//! assert_eq!(recovered, secret);
//! ```
//!
//! ## Zero-Copy Core API
//!
//! For maximum performance and predictable memory usage, use the core API directly:
//!
//! ```
//! use shamir_zero::{shamir_split, shamir_combine, ShamirError};
//!
//! let secret = b"top secret security key";
//! let parts = 5;
//! let threshold = 3;
//!
//! // Pre-allocate exactly once
//! let mut shares = vec![vec![0u8; secret.len() + 1]; parts];
//! let shares_out: Vec<&mut [u8]> = shares.iter_mut().map(|s| s.as_mut_slice()).collect();
//!
//! // Zero-copy split
//! shamir_split(secret, parts, threshold, &mut shares_out).unwrap();
//!
//! // Zero-copy combine
//! let mut recovered = vec![0u8; secret.len()];
//! shamir_combine(&shares[0..threshold].iter().map(|s| s.as_slice()).collect::<Vec<&[u8]>>(), &mut recovered).unwrap();
//!
//! assert_eq!(recovered, secret);
//! ```
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! shamir-zero = { version = "0.1", features = ["fast-inverse"] } # default
//! ```
//!
//! ## License
//!
//! Dual-licensed under MIT or Apache-2.0.
//!
//! [![Crates.io](https://img.shields.io/crates/v/shamir-zero)](https://crates.io/crates/shamir-zero)
//! [![docs.rs](https://img.shields.io/docsrs/shamir-zero)](https://docs.rs/shamir_zero)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
//! [![License: Apache-2.0](https://img.shields.io/badge/License-Apache2.0-red.svg)](LICENSE-APACHE)
//!
//! [Full README](https://github.com/allensarkisyan/shamir_zero)

#![deny(clippy::all)]
#![allow(dead_code, unused)]

mod math;
pub mod shamir;

pub use shamir::ShamirError;
pub use shamir::ShamirZero;
pub use shamir::shamir_combine;
pub use shamir::shamir_split;
