use super::KVector;
use std::simd::Simd;

pub type Vector = KVector<1, 4>;
pub const e1: Vector = Vector {
    components: Simd::from_array([1.0, 0.0, 0.0, 0.0]),
};
pub const e2: Vector = Vector {
    components: Simd::from_array([0.0, 1.0, 0.0, 0.0]),
};
pub const e3: Vector = Vector {
    components: Simd::from_array([0.0, 0.0, 1.0, 0.0]),
};
pub const e0: Vector = Vector {
    components: Simd::from_array([0.0, 0.0, 0.0, 1.0]),
};
