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
        let mut value = term;

        if value == -0.0f64 {
            value = 0.0f64;
        }
        use core::mem::transmute;
        let num: u64 = value.to_bits();
        let signed: i64 = -(unsafe { transmute(num >> 63) });
        let mut mask: u64 = unsafe { transmute(signed) };
        mask |= 0x8000000000000000;
        OrePlaintext(num ^ mask)
    }
}

/// This is useful for debugging purposes only
impl From<OrePlaintext<u64>> for f64 {
    fn from(plaintext: OrePlaintext<u64>) -> f64 {
        let OrePlaintext(term) = plaintext;
        let i = (((term >> 63) as i64) - 1) as u64;
        let mask: u64 = i | 0x8000000000000000;
        f64::from_bits(term ^ mask)
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

    #[test]
    // -0.0 and 0.0 compare as equal in f64's PartialOrd implementation.  This
    // test demonstrates that fact and why we need to filter out -0.0 from the
    // sort_order_is_preserved_for_vec_of_f64_after_converting_to_vec_of_ordered_integer
    // quickcheck test because the sort order will be nondeterministic for
    // vectors containing both 0.0 and -0.0.
    fn zero_and_negative_zero() -> () {
        assert_eq!(0.0f64.partial_cmp(&-0.0f64), Some(Ordering::Equal))
    }

    #[test]
    fn test_subnormal_sorts_correctly() -> () {
        let subnormal = f64::MIN_POSITIVE / 1000.0f64;
        let subnormal_pt = OrePlaintext::<u64>::from(subnormal);
        let zero_pt = OrePlaintext::<u64>::from(0.0f64);
        let min_pt  = OrePlaintext::<u64>::from(f64::MIN_POSITIVE);

        assert_eq!(zero_pt.0.cmp(&min_pt.0), Ordering::Less);
        assert_eq!(zero_pt.0.cmp(&subnormal_pt.0), Ordering::Less);
        assert_eq!(subnormal_pt.0.cmp(&min_pt.0), Ordering::Less)
    }

    #[test]
    fn test_infinity_sorts_correctly() -> () {
        let min_pt = OrePlaintext::<u64>::from(f64::MIN);
        let max_pt = OrePlaintext::<u64>::from(f64::MAX);
        let zero_pt = OrePlaintext::<u64>::from(0.0f64);
        let inf_pt = OrePlaintext::<u64>::from(f64::INFINITY);
        let ninf_pt = OrePlaintext::<u64>::from(f64::NEG_INFINITY);

        assert_eq!(ninf_pt.0.cmp(&min_pt.0), Ordering::Less);
        assert_eq!(ninf_pt.0.cmp(&max_pt.0), Ordering::Less);
        assert_eq!(ninf_pt.0.cmp(&zero_pt.0), Ordering::Less);
        assert_eq!(ninf_pt.0.cmp(&inf_pt.0), Ordering::Less);

        assert_eq!(inf_pt.0.cmp(&min_pt.0), Ordering::Greater);
        assert_eq!(inf_pt.0.cmp(&max_pt.0), Ordering::Greater);
        assert_eq!(inf_pt.0.cmp(&zero_pt.0), Ordering::Greater);
        assert_eq!(inf_pt.0.cmp(&ninf_pt.0), Ordering::Greater);
    }

    #[test]
    fn test_negatives_compare_correctly() -> () {
        let big_pt = OrePlaintext::<u64>::from(-10000f64);
        let sml_pt = OrePlaintext::<u64>::from(-0.001f64);

        assert_eq!(big_pt.0.cmp(&sml_pt.0), Ordering::Less);
    }

    #[test]
    fn test_zeroes_compare_correctly() -> () {
        let neg_pt = OrePlaintext::<u64>::from(-0.0f64);
        let pos_pt = OrePlaintext::<u64>::from(0.0f64);

        assert_eq!(neg_pt.0.cmp(&pos_pt.0), Ordering::Equal);
    }

    quickcheck! {
        fn roundtrip_one_f64(x: f64) -> TestResult {
            if !x.is_nan() && x != -0.0 {
                TestResult::from_bool(x == f64::from(OrePlaintext::<u64>::from(x)))
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
