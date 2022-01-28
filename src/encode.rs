//! This module implements an order-preserving translation of `f32`, `f64`, `u64`,
//! `u32`, `u16`, `u8` to `u64`.
//!
//! The order-preserving nature is only applicable when the source term is `f32`
//! or `f64`.
//!
//! The `u64` that is produced is a plaintext that will be ORE encrypted later
//! on.
//!
//! The mapping is such that the ordering of the floats will be preserved when
//! mapped to an unsigned integer, for example, an array of unsigned integers
//! dervived from a sorted array of doubles will result in no change to its
//! ordering when it itself is sorted.
//!
//! The mapping does not preserve any notion of the previous value after the
//! conversion - only ordering is preserved.
//!
//! Caveat: NaN and -ve & +ve infinity & -0.0 will also be mapped and ordering
//! is not well-defined with those values. Those values should be discarded
//! before converting arrays of those values.
//!
//! This post was used as a reference for building this implementation:
//! https://lemire.me/blog/2020/12/14/converting-floating-point-numbers-to-integers-while-preserving-order
//!
//! # Example
//!
//! ```
//! use ore_encoding_rs::OrderedInteger;
//!
//! let OrderedInteger(encoded) = OrderedInteger::from(123.456f64);
//! ```


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct OrderedInteger(pub u64);

impl From<f32> for OrderedInteger {
    fn from(term: f32) -> OrderedInteger {
        OrderedInteger::from(f64::from(term))
    }
}

impl From<f64> for OrderedInteger {
    fn from(term: f64) -> OrderedInteger {
        use core::mem::transmute;
        let num: u64 = term.to_bits();
        let signed: i64 = -(unsafe { transmute(num >> 63) });
        let mut mask: u64 = unsafe { transmute(signed) };
        mask |= 0x8000000000000000;
        OrderedInteger(num ^ mask)
    }
}

impl From<bool> for OrderedInteger {
    fn from(term: bool) -> OrderedInteger {
        OrderedInteger(term.into())
    }
}

impl From<u8> for OrderedInteger {
    fn from(term: u8) -> OrderedInteger {
        OrderedInteger(term.into())
    }
}

impl From<u16> for OrderedInteger {
    fn from(term: u16) -> OrderedInteger {
        OrderedInteger(term.into())
    }
}

impl From<u32> for OrderedInteger {
    fn from(term: u32) -> OrderedInteger {
        OrderedInteger(term.into())
    }
}

impl From<u64> for OrderedInteger {
    fn from(term: u64) -> OrderedInteger {
        OrderedInteger(term)
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use quickcheck::TestResult;

    fn decode_u64_to_f64(ordered_integer: OrderedInteger) -> f64 {
        let OrderedInteger(term) = ordered_integer;
        let i = (((term >> 63) as i64) - 1) as u64;
        let mask: u64 = i | 0x8000000000000000;
        f64::from_bits(term ^ mask)
    }

    #[test]
    // -0.0 and 0.0 compare as equal in f64's PartialOrd implementation.  This
    // test demonstrates that fact and why we need to filter out -0.0 from the
    // sort_order_is_preserved_for_vec_of_f64_after_converting_to_vec_of_ordered_integer
    // quickcheck test because the sort order will be nondeterministic for
    // vectors containing both 0.0 and -0.0.
    fn zero_and_negative_zero() -> () {
        assert_eq!(0.0f64.partial_cmp(&-0.0f64), Some(Ordering::Equal))
    }

    quickcheck! {
        fn roundtrip_one_f64(x: f64) -> TestResult {
            if !x.is_nan() && x.is_finite() {
                TestResult::from_bool(x == decode_u64_to_f64(x.into()))
            } else {
                TestResult::discard()
            }
        }

        fn sort_order_is_preserved_for_vec_of_f64_after_converting_to_vec_of_ordered_integer(numbers: Vec<f64>) -> TestResult {
            let mut filtered: Vec<f64> = numbers.into_iter().filter(|n| !n.is_nan() && n.is_finite() && *n != -0.0f64).collect();
            filtered.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if filtered.len() == 0 {
                TestResult::discard()
            } else {
                let mut sorted_by_f64 = filtered.clone().into_iter().collect::<Vec<f64>>();
                sorted_by_f64.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut sorted_by_u64 = filtered.clone().into_iter().collect::<Vec<f64>>();
                sorted_by_u64.sort_by(|a, b| OrderedInteger::from(*a).cmp(&OrderedInteger::from(*b)));

                TestResult::from_bool(sorted_by_f64 == sorted_by_u64)
            }
        }
    }
}
