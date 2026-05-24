#![deny(clippy::all)]
#![allow(dead_code, unused)]

mod math;
pub mod shamir;

pub use shamir::shamir_combine;
pub use shamir::shamir_split;
