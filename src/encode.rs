use std::{ops::{Sub, Add}};

use num::{Bounded, One};

/// Convenience alias for all of the traits that must be implemented by the type
/// held by an [OrePlaintext]
pub trait OrePlaintextOps: Ord + Bounded + One + Copy + Sub<Output=Self> + Add<Output=Self> {}

/// Blanket implementation of `OrePlaintextOps`
impl<T: Ord + Sized + Bounded + One + Copy + Sub<Output=T> + Add<Output=T>> OrePlaintextOps for T {}

/// An `OrePlainText` is a wrapper around an unsigned integer which represents a
/// plaintext value before it is encrypted with an ORE encryption scheme.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct OrePlaintext<T>(pub T) where T: OrePlaintextOps;

impl From<f32> for OrePlaintext<u64> {
    fn from(term: f32) -> OrePlaintext<u64> {
        OrePlaintext::from(f64::from(term))
    }
}

impl From<f64> for OrePlaintext<u64> {
    fn from(term: f64) -> OrePlaintext<u64> {
        use core::mem::transmute;
        let num: u64 = term.to_bits();
        let signed: i64 = -(unsafe { transmute(num >> 63) });
        let mut mask: u64 = unsafe { transmute(signed) };
        mask |= 0x8000000000000000;
        OrePlaintext(num ^ mask)
    }
}

impl From<bool> for OrePlaintext<u64> {
    fn from(term: bool) -> OrePlaintext<u64> {
        OrePlaintext(term.into())
    }
}

impl From<u8> for OrePlaintext<u64> {
    fn from(term: u8) -> OrePlaintext<u64> {
        OrePlaintext(term.into())
    }
}

impl From<u16> for OrePlaintext<u64> {
    fn from(term: u16) -> OrePlaintext<u64> {
        OrePlaintext(term.into())
    }
}

impl From<u32> for OrePlaintext<u64> {
    fn from(term: u32) -> OrePlaintext<u64> {
        OrePlaintext(term.into())
    }
}

impl From<u64> for OrePlaintext<u64> {
    fn from(term: u64) -> OrePlaintext<u64> {
        OrePlaintext(term)
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use quickcheck::TestResult;

    fn decode_u64_to_f64(ordered_integer: OrePlaintext<u64>) -> f64 {
        let OrePlaintext(term) = ordered_integer;
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

        fn sort_order_is_preserved_for_vec_of_f64_after_converting_to_vec_of_ore_plaintext(numbers: Vec<f64>) -> TestResult {
            let mut filtered: Vec<f64> = numbers.into_iter().filter(|n| !n.is_nan() && n.is_finite() && *n != -0.0f64).collect();
            filtered.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if filtered.len() == 0 {
                TestResult::discard()
            } else {
                let mut sorted_by_f64 = filtered.clone().into_iter().collect::<Vec<f64>>();
                sorted_by_f64.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut sorted_by_u64 = filtered.clone().into_iter().collect::<Vec<f64>>();
                sorted_by_u64.sort_by(|a, b| OrePlaintext::from(*a).cmp(&OrePlaintext::from(*b)));

                TestResult::from_bool(sorted_by_f64 == sorted_by_u64)
            }
        }
    }
}
