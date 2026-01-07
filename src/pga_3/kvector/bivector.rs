use crate::pga_3::*;
use std::simd::Simd;

pub type Bivector = KVector<2, 6>;

pub const e12: Bivector = Bivector {
    components: Simd::from_array([1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
};
pub const e31: Bivector = Bivector {
    components: Simd::from_array([0.0, 1.0, 0.0, 0.0, 0.0, 0.0]),
};
pub const e23: Bivector = Bivector {
    components: Simd::from_array([0.0, 0.0, 1.0, 0.0, 0.0, 0.0]),
};
pub const e01: Bivector = Bivector {
    components: Simd::from_array([0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
};
pub const e02: Bivector = Bivector {
    components: Simd::from_array([0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
};
pub const e03: Bivector = Bivector {
    components: Simd::from_array([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
};

impl From<(Trivector, Trivector)> for Bivector {
    fn from(tvs: (Trivector, Trivector)) -> Self {
        match tvs.1.is_ideal() {
            true => (tvs.0 + tvs.1) & tvs.0,
            false => tvs.0 & tvs.1,
        }
        .assert::<Bivector>()
    }
}

impl Bivector {
    pub fn exp(&self) -> Motor {
        use std::cmp::Ordering::*;
        match self.eucl_norm().partial_cmp(&0.0) {
            Some(Equal) => Motor::from((1.0, *self, Pseudoscalar(0.0))),
            Some(Greater) => Motor::from((
                self.magnitude().cos(),
                self.normalize() * self.magnitude().sin(),
                Pseudoscalar(0.0),
            )),
            _ => panic!("Magnitude of bivector should be non-negative and non-NaN"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exp() {
        use AnyKVector::*;
        use Versor::*;
        let KVec(Two(bv)) = Vector::from([1.0, 2.0, 0.0, 0.0]) * Vector::from([0.0, 0.0, 3.0, 4.0])
        else {
            panic!("v * v should = bv");
        };
        let expected = Motor::from([
            0.5403022766,
            0.0,
            -0.3763172328,
            0.7526344657,
            -0.5017563701,
            -1.0035127401,
            0.0,
            0.0,
        ]);
        assert_eq!(bv.normalize().exp(), expected);
    }
}
