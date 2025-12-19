use crate::pga_3::multivector::Multivector;
use crate::util::*;
pub use anykvector::AnyKVector;
pub use bivector::Bivector;
pub use pseudoscalar::Pseudoscalar;
pub use scalar::Scalar;
use std::any::Any;
use std::cmp::PartialEq;
use std::ops::{Add, BitAnd, BitXor, Div, Index, IndexMut, Mul, Neg, Sub};
use std::simd::{LaneCount, Simd, SupportedLaneCount, f32x4, simd_swizzle};
use std::slice::SliceIndex;
pub use trivector::Trivector;
pub use vector::Vector;

mod anykvector;
mod bivector;
mod pseudoscalar;
mod scalar;
mod trivector;
mod vector;

#[derive(Debug, Copy, Clone)]
pub struct KVector<const K: u8, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    components: Simd<f32, N>,
}

impl<Idx, const K: u8, const N: usize> Index<Idx> for KVector<K, N>
where
    Idx: SliceIndex<[f32]>,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.components[index]
    }
}

impl<Idx, const K: u8, const N: usize> IndexMut<Idx> for KVector<K, N>
where
    Idx: SliceIndex<[f32], Output = f32>,
    LaneCount<N>: SupportedLaneCount,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl<const K: u8, const N: usize> PartialEq for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl<const K: u8, const N: usize> Add for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            components: self.components + rhs.components,
        }
    }
}

impl<const K: u8, const N: usize> Sub for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            components: self.components - rhs.components,
        }
    }
}

impl<const K: u8, const N: usize> Neg for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            components: -self.components,
        }
    }
}

impl<const K: u8, const N: usize> Mul<Scalar> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn mul(self, other: Scalar) -> Self {
        Self {
            components: self.components * Simd::splat(other),
        }
    }
}

impl<const K: u8, const N: usize> Div<Scalar> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn div(self, other: Scalar) -> Self {
        Self {
            components: self.components / Simd::splat(other),
        }
    }
}

impl<const K: u8, const G: u8, const N: usize, const D: usize> BitXor<KVector<G, D>>
    for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
    LaneCount<D>: SupportedLaneCount,
{
    type Output = AnyKVector;

    #[inline]
    fn bitxor(self, rhs: KVector<G, D>) -> Self::Output {
        self.outer(rhs)
    }
}

impl<const K: u8, const N: usize> BitXor<AnyKVector> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = AnyKVector;

    #[inline]
    fn bitxor(self, rhs: AnyKVector) -> Self::Output {
        match rhs {
            AnyKVector::Zero(s) => (self * s).into(),
            AnyKVector::One(v) => (self ^ v).into(),
            AnyKVector::Two(bv) => (self ^ bv).into(),
            AnyKVector::Three(tv) => (self ^ tv).into(),
            AnyKVector::Four(_) => AnyKVector::Zero(0.0),
        }
    }
}

impl<const K: u8, const G: u8, const N: usize, const D: usize> BitAnd<KVector<G, D>>
    for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
    LaneCount<D>: SupportedLaneCount,
{
    type Output = AnyKVector;

    #[inline]
    fn bitand(self, rhs: KVector<G, D>) -> Self::Output {
        self.regressive(rhs)
    }
}

impl<const K: u8, const N: usize> BitAnd<AnyKVector> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = AnyKVector;

    fn bitand(self, rhs: AnyKVector) -> Self::Output {
        (self.dual() ^ rhs.dual()).undual()
    }
}

impl<const K: u8, const N: usize> From<[f32; N]> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(value: [f32; N]) -> Self {
        Self {
            components: Simd::from(value),
        }
    }
}

impl<const K: u8, const N: usize> From<Simd<f32, N>> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(value: Simd<f32, N>) -> Self {
        Self { components: value }
    }
}

impl<const K: u8, const N: usize> Multivector for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn reverse(&self) -> Self {
        *self * Self::reverse_multiplier()
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        *self * (-1_i32.pow(Self::grade() as u32) as Scalar)
    }
}

impl<const K: u8, const N: usize> KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    pub fn grade() -> u8 {
        K
    }

    // basis is a binary value where the bits, from right-to-
    // left, represent e3, e2, e1, and e0 respectively being
    // present in the basis being requested
    #[inline]
    pub fn e(&self, basis: u8) -> f32 {
        match K {
            1 => match basis {
                0b0100 => self[0],
                0b0010 => self[1],
                0b0001 => self[2],
                0b1000 => self[3],
                _ => 0.0,
            },
            2 => match basis {
                0b0110 => self[0],
                0b0101 => self[1],
                0b0011 => self[2],
                0b1100 => self[3],
                0b1010 => self[4],
                0b1001 => self[5],
                _ => 0.0,
            },
            3 => match basis {
                0b0111 => self[0],
                0b1011 => self[1],
                0b1101 => self[2],
                0b1110 => self[3],
                _ => 0.0,
            },
            _ => 0.0,
        }
    }

    #[inline]
    pub fn is_ideal(&self) -> bool {
        self[0..Self::ideal_index()]
            .iter()
            .fold(true, |acc, f| acc && float_eq(*f, 0.0))
    }

    #[inline]
    const fn ideal_index() -> usize {
        match K {
            1 => 3,
            2 => 3,
            3 => 1,
            _ => 0,
        }
    }

    #[inline]
    pub fn eucl_norm(&self) -> Scalar {
        match K {
            3 => self[0].abs(),
            _ => sum_of_squares(self.real_part()),
        }
    }

    #[inline]
    pub fn ideal_norm(&self) -> Scalar {
        match K {
            1 => self[3].abs(),
            _ => sum_of_squares(self.ideal_part()),
        }
    }

    #[inline]
    pub fn magnitude(&self) -> Scalar {
        match self.eucl_norm() {
            0.0 => self.ideal_norm(),
            m => m,
        }
    }

    #[inline]
    pub fn normalize(&mut self) -> &Self {
        *self = *self / self.magnitude();
        self
    }

    #[inline]
    const fn reverse_multiplier() -> Scalar {
        -1_i32.pow((K * (K - 1)) as u32) as Scalar
    }

    #[inline]
    pub fn real_part(&self) -> &[f32] {
        &self[0..Self::ideal_index()]
    }

    #[inline]
    pub fn ideal_part(&self) -> &[f32] {
        &self[Self::ideal_index()..N]
    }

    #[inline]
    pub fn dual(self) -> AnyKVector {
        match K {
            1 => AnyKVector::Three(Trivector::from(simd_swizzle!(
                self.components,
                [3, 0, 1, 2]
            ))),
            2 => AnyKVector::Two(Bivector::from([
                self[5], self[4], self[3], self[2], self[1], self[0],
            ])),
            3 => AnyKVector::One(Vector::from(-simd_swizzle!(self.components, [1, 2, 3, 0]))),
            _ => AnyKVector::Zero(0.0),
        }
    }

    #[inline]
    pub fn undual(self) -> AnyKVector {
        match K {
            1 => AnyKVector::Three(Trivector::from(-simd_swizzle!(
                self.components,
                [3, 0, 1, 2]
            ))),
            2 => AnyKVector::Two(Bivector::from([
                self[5], self[4], self[3], self[2], self[1], self[0],
            ])),
            3 => AnyKVector::One(Vector::from(simd_swizzle!(self.components, [1, 2, 3, 0]))),
            _ => AnyKVector::Zero(0.0),
        }
    }

    pub fn inner(self, other: Self) -> Scalar {
        (self.components * other.components * Simd::splat(Self::reverse_multiplier()))
            [0..Self::ideal_index()]
            .iter()
            .fold(0.0, |acc, f| acc + f)
    }

    pub fn outer<const G: u8, const D: usize>(self, rhs: KVector<G, D>) -> AnyKVector
    where
        LaneCount<D>: SupportedLaneCount,
    {
        match K + G {
            2 => {
                const I1: [usize; 6] = [0, 2, 1, 3, 3, 3];
                const I2: [usize; 6] = [1, 0, 2, 0, 1, 2];
                let t1 = simd_swizzle!(self.components, I1);
                let t2 = simd_swizzle!(rhs.components, I2);
                let t3 = simd_swizzle!(self.components, I2);
                let t4 = simd_swizzle!(rhs.components, I1);
                AnyKVector::Two(Bivector::from(t1 * t2 - t3 * t4))
            }
            3 => {
                let t1 = f32x4::from([
                    self.e(0b0100) * rhs.e(0b0011),
                    -self.e(0b1000) * rhs.e(0b0011),
                    -self.e(0b1000) * rhs.e(0b0101),
                    -self.e(0b1000) * rhs.e(0b0110),
                ]);
                let t2 = f32x4::from([
                    self.e(0b0010) * rhs.e(0b0101),
                    self.e(0b0010) * rhs.e(0b1001),
                    -self.e(0b0100) * rhs.e(0b1001),
                    self.e(0b0100) * rhs.e(0b1010),
                ]);
                let t3 = f32x4::from([
                    self.e(0b0001) * rhs.e(0b0110),
                    -self.e(0b0001) * rhs.e(0b1010),
                    self.e(0b0001) * rhs.e(0b1100),
                    -self.e(0b0010) * rhs.e(0b1100),
                ]);
                let t4 = f32x4::from([
                    self.e(0b0110) * rhs.e(0b0001),
                    -self.e(0b0011) * rhs.e(0b1000),
                    -self.e(0b0101) * rhs.e(0b1000),
                    -self.e(0b0110) * rhs.e(0b1000),
                ]);
                let t5 = f32x4::from([
                    self.e(0b0101) * rhs.e(0b0010),
                    self.e(0b1001) * rhs.e(0b0010),
                    -self.e(0b1001) * rhs.e(0b0100),
                    self.e(0b1010) * rhs.e(0b0100),
                ]);
                let t6 = f32x4::from([
                    self.e(0b0011) * rhs.e(0b0100),
                    -self.e(0b1010) * rhs.e(0b0001),
                    self.e(0b1100) * rhs.e(0b0001),
                    -self.e(0b1100) * rhs.e(0b0010),
                ]);
                AnyKVector::Three(Trivector::from(t1 + t2 + t3 + t4 + t5 + t6))
            }
            4 => match G as i8 - K as i8 {
                0 => AnyKVector::Four(Pseudoscalar(
                    self[0] * rhs[5]
                        + self[1] * rhs[4]
                        + self[2] * rhs[3]
                        + self[3] * rhs[2]
                        + self[4] * rhs[1]
                        + self[5] * rhs[0],
                )),
                _ => AnyKVector::Four(Pseudoscalar(
                    self.e(0b1000) * rhs.e(0b0111)
                        + self.e(0b0100) * rhs.e(0b1011)
                        + self.e(0b0010) * rhs.e(0b1101)
                        + self.e(0b0001) * rhs.e(0b1110)
                        - self.e(0b0111) * rhs.e(0b1000)
                        - self.e(0b1011) * rhs.e(0b0100)
                        - self.e(0b1101) * rhs.e(0b0010)
                        - self.e(0b1110) * rhs.e(0b0001),
                )),
            },
            _ => AnyKVector::Zero(0.0),
        }
    }

    #[inline]
    pub fn regressive<const G: u8, const D: usize>(self, rhs: KVector<G, D>) -> AnyKVector
    where
        LaneCount<D>: SupportedLaneCount,
    {
        (self.dual() ^ rhs.dual()).undual()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outer_vector_vector() {
        let v1 = Vector::from([2.0, 3.0, 4.0, 1.0]);
        let v2 = Vector::from([3.0, 2.0, 1.0, 4.0]);
        if let AnyKVector::Two(bv) = v1 ^ v2 {
            assert_eq!(bv, Bivector::from([-5.0, 10.0, -5.0, -5.0, -10.0, -15.0]));
        } else {
            panic!("v1 ^ v2 was not a bivector");
        }
    }

    #[test]
    fn outer_vector_bivector() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let bv = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        if let AnyKVector::Three(tv) = v ^ bv {
            assert_eq!(tv, Trivector::from([10.0, -15.0, -2.0, -7.0]));
        } else {
            panic!("v ^ bv was not a trivector");
        }

        if let AnyKVector::Three(tv) = bv ^ v {
            assert_eq!(tv, Trivector::from([10.0, -15.0, -2.0, -7.0]));
        } else {
            panic!("bv ^ v was not a trivector");
        }
    }

    #[test]
    fn outer_bivector_bivector() {
        let bv1 = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let bv2 = Bivector::from([6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

        if let AnyKVector::Four(Pseudoscalar(n)) = bv1 ^ bv2 {
            assert!(float_eq(n, 91.0));
        } else {
            panic!("bv1 ^ bv2 was not a pseudoscalar");
        }

        if let AnyKVector::Four(Pseudoscalar(n)) = bv2 ^ bv1 {
            assert!(float_eq(n, 91.0))
        } else {
            panic!("bv2 ^ bv1 was not a pseudoscalar");
        }
    }

    #[test]
    fn outer_vector_trivector() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let tv = Trivector::from([4.0, 3.0, 2.0, 1.0]);

        if let AnyKVector::Four(Pseudoscalar(n)) = v ^ tv {
            assert!(float_eq(n, 26.0));
        } else {
            panic!("v ^ tv was not a pseudoscalar");
        }

        if let AnyKVector::Four(Pseudoscalar(n)) = tv ^ v {
            assert!(float_eq(n, -26.0));
        } else {
            panic!("tv ^ v was not a pseudoscalar");
        }
    }

    #[test]
    fn outer_zeros() {
        let bv = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let tv = Trivector::from([1.0, 2.0, 3.0, 4.0]);

        if let AnyKVector::Zero(n) = bv ^ tv {
            assert!(float_eq(n, 0.0));
        } else {
            panic!("bv ^ tv was not a scalar");
        }

        if let AnyKVector::Zero(n) = tv ^ bv {
            assert!(float_eq(n, 0.0));
        } else {
            panic!("tv ^ bv was not a scalar");
        }

        if let AnyKVector::Zero(n) = tv ^ tv {
            assert!(float_eq(n, 0.0));
        } else {
            panic!("tv ^ tv was not a scalar");
        }
    }

    #[test]
    fn undual_inverse_of_dual() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let bv = Bivector::from([6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let tv = Trivector::from([1.0, 2.0, 3.0, 4.0]);

        assert_eq!(v.dual().undual(), v.into());
        assert_eq!(v.undual().dual(), v.into());
        assert_eq!(bv.dual().undual(), bv.into());
        assert_eq!(bv.undual().dual(), bv.into());
        assert_eq!(tv.dual().undual(), tv.into());
        assert_eq!(tv.undual().dual(), tv.into());
    }

    #[test]
    fn dual_vector_trivector() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v.dual(), Trivector::from([4.0, 1.0, 2.0, 3.0]).into());

        let tv = Trivector::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(tv.dual(), Vector::from([-2.0, -3.0, -4.0, -1.0]).into());
    }

    #[test]
    fn dual_bivector() {
        let bv = Bivector::from([6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        assert_eq!(
            bv.dual(),
            Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).into()
        );
    }

    #[test]
    fn regressive_zero() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let bv = Bivector::from([6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

        if let AnyKVector::Zero(n) = v & bv {
            assert!(float_eq(n, 0.0));
        } else {
            panic!("v & bv was not a scalar");
        }
    }

    #[test]
    fn regressive_bivector_trivector() {
        let bv = Bivector::from([6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let tv = Trivector::from([1.0, 2.0, 3.0, 4.0]);

        if let AnyKVector::One(v) = tv & bv {
            assert_eq!(v, Vector::from([5.0, -2.0, 3.0, -16.0]));
        } else {
            panic!("tv & bv was not a vector");
        }

        if let AnyKVector::One(v) = bv & tv {
            assert_eq!(v, Vector::from([5.0, -2.0, 3.0, -16.0]));
        } else {
            panic!("bv & tv was not a vector");
        }
    }

    #[test]
    fn regressive_trivector_trivector() {
        let tv1 = Trivector::from([1.0, 2.0, 3.0, 4.0]);
        let tv2 = Trivector::from([4.0, 3.0, 2.0, 1.0]);

        if let AnyKVector::Two(bv) = tv1 & tv2 {
            assert_eq!(bv, Bivector::from([-15.0, -10.0, -5.0, -5.0, 10.0, -5.0]));
        } else {
            panic!("tv1 & tv2 was not a vector");
        }
    }
}
