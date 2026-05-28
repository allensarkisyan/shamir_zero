// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (C) 2026 Allen Sarkisyan

#![deny(clippy::all)]
#![allow(dead_code, unused)]

mod math;
pub mod shamir;

pub use shamir::ShamirError;
pub use shamir::shamir_combine;
pub use shamir::shamir_split;
