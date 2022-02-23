use crate::{OrePlaintext, OrePlaintextOps};

pub struct OreRange<T> where T: OrePlaintextOps {
    pub min: OrePlaintext<T>,
    pub max: OrePlaintext<T>,
}

pub fn encode_between<T>(min: OrePlaintext<T>, max: OrePlaintext<T>) -> OreRange<T> where T: OrePlaintextOps {
    OreRange{ min, max }
}

pub fn encode_lt<T>(value: OrePlaintext<T>) -> OreRange<T> where T: OrePlaintextOps {
    let OrePlaintext(upper) = value;
    OreRange{
        min: OrePlaintext(T::min_value()),
        max: OrePlaintext(
            T::max(upper - T::one(), T::min_value())
        )
    }
}

pub fn encode_lte<T>(value: OrePlaintext<T>) -> OreRange<T> where T: OrePlaintextOps {
    OreRange{ min: OrePlaintext(T::min_value()), max: value }
}

pub fn encode_gt<T>(value: OrePlaintext<T>) -> OreRange<T> where T: OrePlaintextOps {
    let OrePlaintext(lower) = value;
    OreRange{
        min: OrePlaintext(
            T::min(lower + T::one(), T::max_value())
        ),
        max: OrePlaintext(T::max_value())
     }
}

pub fn encode_gte<T>(value: OrePlaintext<T>) -> OreRange<T> where T: OrePlaintextOps {
    OreRange{ min: value, max: OrePlaintext(T::max_value()) }
}

pub fn encode_eq<T>(value: OrePlaintext<T>) -> OreRange<T> where T: OrePlaintextOps {
    OreRange{ min: value, max: value }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OrePlaintext;

    #[test]
    fn test_encode_eq() -> () {
        let pt = OrePlaintext(0u64);
        let range = encode_eq(pt);
        assert_eq!(range.min, range.max)
    }

    #[test]
    fn test_encode_lt() -> () {
        let pt = OrePlaintext(100u64);
        let range = encode_lt(pt);
        assert_eq!(range.min, OrePlaintext(0u64));
        assert_eq!(range.max, OrePlaintext(99u64))
    }

    #[test]
    fn test_encode_lte() -> () {
        let pt = OrePlaintext(100u64);
        let range = encode_lte(pt);
        assert_eq!(range.min, OrePlaintext(0u64));
        assert_eq!(range.max, OrePlaintext(100u64))
    }

    #[test]
    fn test_encode_gt() -> () {
        let pt = OrePlaintext(100u64);
        let range = encode_gt(pt);
        assert_eq!(range.min, OrePlaintext(101u64));
        assert_eq!(range.max, OrePlaintext(std::u64::MAX))
    }

    #[test]
    fn test_encode_gte() -> () {
        let pt = OrePlaintext(100u64);
        let range = encode_gte(pt);
        assert_eq!(range.min, OrePlaintext(100u64));
        assert_eq!(range.max, OrePlaintext(std::u64::MAX))
    }
}