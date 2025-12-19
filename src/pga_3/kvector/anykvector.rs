use super::KVector;
use super::{Bivector, Pseudoscalar, Scalar, Trivector, Vector};
use crate::util::float_eq;
use AnyKVector::*;
use std::any::Any;
use std::ops::{BitAnd, BitXor, Div, Mul, Neg};
use std::simd::{LaneCount, SupportedLaneCount};

#[derive(Copy, Clone, Debug)]
pub enum AnyKVector {
    Zero(Scalar),
    One(Vector),
    Two(Bivector),
    Three(Trivector),
    Four(Pseudoscalar),
}

impl PartialEq for AnyKVector {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Zero(f1), Zero(f2)) => float_eq(*f1, *f2),
            (One(v1), One(v2)) => v1 == v2,
            (Two(bv1), Two(bv2)) => bv1 == bv2,
            (Three(tv1), Three(tv2)) => tv1 == tv2,
            (Four(ps1), Four(ps2)) => ps1 == ps2,
            _ => false,
        }
    }
}

impl Neg for AnyKVector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Zero(s) => Zero(-s),
            One(v) => One(-v),
            Two(bv) => Two(-bv),
            Three(tv) => Three(-tv),
            Four(ps) => Four(-ps),
        }
    }
}

impl Mul<Scalar> for AnyKVector {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        match self {
            Zero(s) => Zero(s * rhs),
            One(v) => One(v * rhs),
            Two(bv) => Two(bv * rhs),
            Three(tv) => Three(tv * rhs),
            Four(ps) => Four(ps * rhs),
        }
    }
}

impl Div<Scalar> for AnyKVector {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        match self {
            Zero(s) => Zero(s / rhs),
            One(v) => One(v / rhs),
            Two(bv) => Two(bv / rhs),
            Three(tv) => Three(tv / rhs),
            Four(ps) => Four(ps / rhs),
        }
    }
}

impl<const K: u8, const N: usize> BitXor<KVector<K, N>> for AnyKVector
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: KVector<K, N>) -> Self::Output {
        match self {
            Zero(s) => (rhs * s).into(),
            One(v) => (v ^ rhs).into(),
            Two(bv) => (bv ^ rhs).into(),
            Three(tv) => (tv ^ rhs).into(),
            Four(_) => Zero(0.0),
        }
    }
}

impl BitXor for AnyKVector {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.outer(rhs)
    }
}

impl<const K: u8, const N: usize> BitAnd<KVector<K, N>> for AnyKVector
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: KVector<K, N>) -> Self::Output {
        (self.dual() ^ rhs.dual()).undual()
    }
}

impl BitAnd for AnyKVector {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: AnyKVector) -> Self::Output {
        (self.dual() ^ rhs.dual()).undual()
    }
}

impl<const K: u8, const N: usize> From<KVector<K, N>> for AnyKVector
where
    LaneCount<N>: SupportedLaneCount,
{
    fn from(value: KVector<K, N>) -> AnyKVector {
        if let Some(vector) = (&value as &dyn Any).downcast_ref::<Vector>() {
            return One(*vector);
        }
        if let Some(bivector) = (&value as &dyn Any).downcast_ref::<Bivector>() {
            return Two(*bivector);
        }
        if let Some(trivector) = (&value as &dyn Any).downcast_ref::<Trivector>() {
            return Three(*trivector);
        }
        return Zero(0.0);
    }
}

impl From<Scalar> for AnyKVector {
    fn from(value: Scalar) -> Self {
        Zero(value)
    }
}

impl From<Pseudoscalar> for AnyKVector {
    fn from(value: Pseudoscalar) -> Self {
        Four(value)
    }
}

impl AnyKVector {
    pub fn grade(&self) -> u8 {
        match self {
            Zero(_) => 0,
            One(_) => 1,
            Two(_) => 2,
            Three(_) => 3,
            Four(_) => 4,
        }
    }

    pub fn dual(self) -> Self {
        match self {
            Zero(s) => {
                if s != 0.0 {
                    Four(Pseudoscalar(s))
                } else {
                    self
                }
            }
            One(v) => v.dual(),
            Two(bv) => bv.dual(),
            Three(tv) => tv.dual(),
            Four(ps) => Zero(ps.0),
        }
    }

    pub fn undual(self) -> Self {
        match self {
            One(v) => v.undual(),
            Three(tv) => tv.undual(),
            _ => self.dual(),
        }
    }

    pub fn outer(self, rhs: Self) -> Self {
        match self {
            Zero(s) => rhs * s,
            One(v) => v ^ rhs,
            Two(bv) => bv ^ rhs,
            Three(tv) => tv ^ rhs,
            Four(ps) => {
                if let Zero(s) = rhs {
                    Four(ps * s)
                } else {
                    Zero(0.0)
                }
            }
        }
    }
}
