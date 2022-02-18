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