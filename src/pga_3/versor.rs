use crate::pga_3::*;
pub use motor::*;
pub use odd_versor::*;
use std::ops::{Div, Mul, Neg};
use std::simd::{LaneCount, SupportedLaneCount};

mod motor;
mod odd_versor;

const VERSOR_ZERO_EPSILON: f32 = 0.001;

#[derive(Debug, Copy, Clone)]
pub enum Versor {
    Even(Motor),
    Odd(OddVersor),
    KVec(AnyKVector),
}

fn is_zero(slice: &[f32]) -> bool {
    slice
        .iter()
        .fold(true, |acc, f| acc && f.abs() < VERSOR_ZERO_EPSILON)
}

impl From<AnyKVector> for Versor {
    #[inline]
    fn from(kv: AnyKVector) -> Self {
        Self::KVec(kv)
    }
}

impl<const K: u8, const N: usize> From<KVector<K, N>> for Versor
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(kv: KVector<K, N>) -> Self {
        /*
        if !is_zero(&kv[0..N]) {
        */
        Self::KVec(kv.into())
        /*
        } else {
            Self::KVec(AnyKVector::Zero(0.0))
        }
        */
    }
}

impl From<Scalar> for Versor {
    #[inline]
    fn from(s: Scalar) -> Self {
        Versor::KVec(s.into())
    }
}

impl From<Pseudoscalar> for Versor {
    #[inline]
    fn from(ps: Pseudoscalar) -> Self {
        Versor::KVec(ps.into())
    }
}

impl From<OddVersor> for Versor {
    #[inline]
    fn from(ov: OddVersor) -> Self {
        if ov.zero() {
            Versor::KVec(0.0.into())
        } else if is_zero(&ov[0..4]) {
            Versor::KVec(Trivector::from([ov[4], ov[5], ov[6], ov[7]]).into())
        } else if is_zero(&ov[4..8]) {
            Versor::KVec(Vector::from([ov[0], ov[1], ov[2], ov[3]]).into())
        } else {
            Versor::Odd(ov)
        }
    }
}

impl From<Motor> for Versor {
    #[inline]
    fn from(m: Motor) -> Self {
        use crate::util::float_eq;
        if is_zero(&m[1..8]) {
            Versor::KVec(m[0].into())
        } else if is_zero(&m[0..7]) {
            Versor::KVec(Pseudoscalar(m[7]).into())
        } else if float_eq(m[0], 0.0) && float_eq(m[7], 0.0) {
            Versor::KVec(Bivector::from([m[1], m[2], m[3], m[4], m[5], m[6]]).into())
        } else {
            Versor::Even(m)
        }
    }
}

impl PartialEq for Versor {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        use Versor::*;
        match (self, other) {
            (Odd(ov1), Odd(ov2)) => ov1 == ov2,
            (Even(m1), Even(m2)) => m1 == m2,
            (KVec(kv1), KVec(kv2)) => kv1 == kv2,
            _ => false,
        }
    }
}

impl Neg for Versor {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        use Versor::*;
        match self {
            Odd(ov) => Odd(-ov),
            Even(m) => Even(-m),
            KVec(kv) => KVec(-kv),
        }
    }
}

impl Mul<Scalar> for Versor {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Scalar) -> Self::Output {
        use Versor::*;
        match self {
            Odd(ov) => Odd(ov * rhs),
            Even(m) => Even(m * rhs),
            KVec(kv) => KVec(kv * rhs),
        }
    }
}

impl<T: Multivector + NonScalar> Mul<T> for Versor {
    type Output = Versor;
    #[inline]
    fn mul(self, rhs: T) -> Versor {
        self.geo(rhs)
    }
}

impl Div<Scalar> for Versor {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Scalar) -> Self::Output {
        use Versor::*;
        match self {
            Odd(ov) => Odd(ov / rhs),
            Even(m) => Even(m / rhs),
            KVec(kv) => KVec(kv / rhs),
        }
    }
}

impl Multivector for Versor {
    #[inline]
    fn e(&self, basis: u8) -> f32 {
        use Versor::*;
        match self {
            Odd(ov) => ov.e(basis),
            Even(m) => m.e(basis),
            KVec(kv) => kv.e(basis),
        }
    }

    #[inline]
    fn grade(&self, g: u8) -> AnyKVector {
        use Versor::*;
        match self {
            Odd(ov) => ov.grade(g),
            Even(m) => m.grade(g),
            KVec(kv) => kv.grade(g),
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        use Versor::*;
        match self {
            Odd(ov) => ov.highest_grade(),
            Even(m) => m.highest_grade(),
            KVec(kv) => kv.highest_grade(),
        }
    }

    #[inline]
    fn zero(&self) -> bool {
        use Versor::*;
        match self {
            Odd(ov) => ov.zero(),
            Even(m) => m.zero(),
            KVec(kv) => kv.zero(),
        }
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        use Versor::*;
        match self {
            Odd(ov) => ov.is_ideal(),
            Even(m) => m.is_ideal(),
            KVec(kv) => kv.is_ideal(),
        }
    }

    #[inline]
    fn reverse(&self) -> Self {
        use Versor::*;
        match self {
            Odd(ov) => Odd(ov.reverse()),
            Even(m) => Even(m.reverse()),
            KVec(kv) => KVec(kv.reverse()),
        }
    }

    #[inline]
    fn normalize(self) -> Self {
        use Versor::*;
        match self {
            Odd(ov) => Odd(ov.normalize()),
            Even(m) => Even(m.normalize()),
            KVec(kv) => KVec(kv.normalize()),
        }
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        use Versor::*;
        match self {
            Odd(ov) => Odd(-*ov),
            Even(m) => Even(*m),
            KVec(kv) => KVec(kv.grade_involution()),
        }
    }

    #[inline]
    fn dual(self) -> Versor {
        use Versor::*;
        match self {
            Odd(ov) => ov.dual(),
            Even(m) => m.dual(),
            KVec(kv) => kv.dual(),
        }
    }

    #[inline]
    fn undual(self) -> Versor {
        use Versor::*;
        match self {
            Odd(ov) => ov.undual(),
            Even(m) => m.undual(),
            KVec(kv) => kv.undual(),
        }
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        use Versor::*;
        match self {
            Odd(ov) => ov.geo(rhs),
            Even(m) => m.geo(rhs),
            KVec(kv) => kv.geo(rhs),
        }
    }
}

impl Versor {
    pub fn assert<T: SingleGrade + 'static>(&self) -> T {
        use Versor::*;
        match self {
            Even(_) => panic!("Assert failed: motor not kvector"),
            Odd(_) => panic!("Assert failed: oddvector not kvector"),
            KVec(kv) => kv.assert::<T>(),
        }
    }
}
