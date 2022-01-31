//! The order-revealing encryption scheme of [docs.rs/ore-rs](https://docs.rs/ore-rs) works on
//! plaintexts of type `u64`. The encryption scheme allows ciphertexts to be
//! compared to each other in such that two ciphertexts will reveal the ordering
//! relationship of their plaintexts.
//!
//! This is great when your plain texts are u64 but if your plaintext is not a
//! `u64` then you will need to map your plaintext domain to a corresponding
//! `u64` in such a way as to preserve ordering relationships of the original
//! plaintext.
//!
//! This module defines a type `OrePlaintext<T>` and [From] implementations
//! on [f32], [f64], [u8], [bool], [u16], [u32] & [u64] for `OrderedInteger<u64>`.
//!
//! The mapping is technically reversible (no information is lost) but a reverse
//! mapping function is not provided.
//!
//! Caveat: NaN and -ve & +ve infinity & -0.0 will also be mapped and ordering
//! is not well-defined with those values. Those values should be discarded
//! before converting vectors of those values.
//!
//! This post was used as a reference for building this implementation
//! [Converting floating point numbers to integers while preserving order](https://lemire.me/blog/2020/12/14/converting-floating-point-numbers-to-integers-while-preserving-order)
//!
//! # Example
//!
//! ```
//! use ore_encoding_rs::OrePlaintext;
//!
//! let OrePlaintext(encoded) = OrePlaintext::from(123.456f64);
//! ```

mod encode;
mod siphash;

pub use encode::*;
pub use siphash::*;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;