use crate::pga_3::*;
use crate::util::*;
pub use anykvector::*;
pub use bivector::*;
pub use pseudoscalar::*;
pub use scalar::*;
use std::any::Any;
use std::cmp::PartialEq;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Sub};
use std::simd::{LaneCount, Simd, SupportedLaneCount, f32x4, simd_swizzle};
use std::slice::SliceIndex;
pub use trivector::*;
pub use vector::*;

#[macro_use]
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
        self.components
            .as_array()
            .iter()
            .enumerate()
            .fold(true, |acc, (i, e)| acc && float_eq(*e, other[i]))
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

impl<const K: u8, const N: usize, T> Mul<T> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: Multivector,
    T: NonScalar,
{
    type Output = Versor;
    #[inline]
    fn mul(self, other: T) -> Versor {
        self.geo(other)
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

impl<const K: u8, const N: usize, T> BitXor<T> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SingleGrade,
{
    type Output = AnyKVector;

    #[inline]
    fn bitxor(self, rhs: T) -> Self::Output {
        self.outer(rhs)
    }
}

impl<const K: u8, const N: usize, T> BitAnd<T> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SingleGrade,
{
    type Output = AnyKVector;

    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        self.regressive(rhs)
    }
}

impl<const K: u8, const N: usize, T> BitOr<T> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
    T: SingleGrade,
{
    type Output = AnyKVector;

    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        self.inner(rhs)
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
        *self * (-1_i32).pow(K as u32) as Scalar
    }

    // basis is a binary value where the bits, from right-to-
    // left, represent e3, e2, e1, and e0 respectively being
    // present in the basis being requested
    #[inline]
    fn e(&self, basis: u8) -> f32 {
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
    fn grade(&self, g: u8) -> AnyKVector {
        if g == K {
            (*self).into()
        } else {
            AnyKVector::Zero(0.0)
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        K
    }

    #[inline]
    fn zero(&self) -> bool {
        self.components == Simd::splat(0.0)
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        self[0..Self::ideal_index()]
            .iter()
            .fold(true, |acc, f| acc && float_eq(*f, 0.0))
    }

    #[inline]
    fn dual(self) -> Versor {
        match K {
            1 => Versor::KVec(AnyKVector::Three(Trivector::from(simd_swizzle!(
                self.components,
                [3, 0, 1, 2]
            )))),
            2 => Versor::KVec(AnyKVector::Two(Bivector::from([
                self[5], self[4], self[3], self[2], self[1], self[0],
            ]))),
            3 => Versor::KVec(AnyKVector::One(Vector::from(-simd_swizzle!(
                self.components,
                [1, 2, 3, 0]
            )))),
            _ => Versor::from(0.0),
        }
    }

    #[inline]
    fn undual(self) -> Versor {
        match K {
            1 => -self.dual(),
            2 => self.dual(),
            3 => -self.dual(),
            _ => Versor::from(0.0),
        }
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        use AnyKVector::*;
        use Versor::*;
        match rhs.into() {
            Even(m) => {
                let Two(rhs_g2) = m.grade(2) else {
                    panic!("Grade 2 of motor should be a bivector")
                };
                match K % 2 {
                    0 => {
                        let t1 = {
                            let Two(bv) = (self * m.e(0b0000)).into() else {
                                panic!("Scaled bivector should be a bivector");
                            };
                            Motor::from(bv)
                        };
                        let t2 = match rhs_g2.reverse_geo_kvector(self) {
                            Even(m) => m,
                            KVec(kv) => match kv {
                                Zero(s) => Motor::from(s),
                                Two(bv) => Motor::from(bv),
                                Four(ps) => Motor::from(ps),
                                _ => panic!("Invalid product of bivectors"),
                            },
                            _ => panic!("Bivector * Bivector should be a motor"),
                        };
                        let t3 = {
                            let bv = match self.inner(m.grade(4)) {
                                Zero(0.0) => Bivector::from([0.0; 6]),
                                Two(bv) => bv,
                                _ => panic!("Bivector | Pseudoscalar should be a bivector"),
                            };
                            Motor::from(bv)
                        };
                        Versor::from(t1 + t2 + t3)
                    }
                    1 => {
                        let t1 = match (self * m.e(0b0000)).into() {
                            One(v) => OddVersor::from(v),
                            Three(tv) => OddVersor::from(tv),
                            _ => panic!("Scaled odd K-vector should still be odd K-vector"),
                        };
                        let t2 = match rhs_g2.reverse_geo_kvector(self) {
                            Odd(ov) => ov,
                            KVec(Zero(0.0)) => OddVersor::from([0.0; 8]),
                            KVec(One(v)) => OddVersor::from(v),
                            KVec(Three(tv)) => OddVersor::from(tv),
                            _ => panic!("Odd K-Vector * Motor should be an odd versor"),
                        };
                        let t3 = match self.inner(m.grade(4)) {
                            One(v) => OddVersor::from(v),
                            Three(tv) => OddVersor::from(tv),
                            _ => {
                                panic!(
                                    "Odd K-Vector * Pseudoscalar should still be an odd K-vector"
                                )
                            }
                        };
                        Versor::from(t1 + t2 + t3)
                    }
                    _ => panic!("Anything mod 2 should be 0 or 1"),
                }
            }
            Odd(ov) => {
                let One(rhs_g1) = ov.grade(1) else {
                    panic!("Grade 1 of odd versor should be a trivector");
                };
                let Three(rhs_g3) = ov.grade(3) else {
                    panic!("Grade 3 of odd versor should be a trivector");
                };
                match K % 2 {
                    0 => {
                        use std::simd::f32x8;
                        let t1 = match rhs_g1.reverse_geo_kvector(self) {
                            Odd(ov) => ov,
                            KVec(Zero(0.0)) => OddVersor::from(f32x8::splat(0.0)),
                            KVec(One(v)) => OddVersor::from(v),
                            KVec(Three(tv)) => OddVersor::from(tv),
                            _ => panic!("vector * bivector should be an odd versor"),
                        };
                        let t2 = match rhs_g3.reverse_geo_kvector(self) {
                            Odd(ov) => ov,
                            KVec(Zero(0.0)) => OddVersor::from(f32x8::splat(0.0)),
                            KVec(One(v)) => OddVersor::from(v),
                            KVec(Three(tv)) => OddVersor::from(tv),
                            _ => panic!("trivector * bivector should be an odd versor"),
                        };
                        Versor::from(t1 + t2)
                    }
                    1 => {
                        let t1 = match rhs_g1.reverse_geo_kvector(self) {
                            Even(m) => m,
                            KVec(Zero(s)) => Motor::from(s),
                            KVec(Two(bv)) => Motor::from(bv),
                            KVec(Four(ps)) => Motor::from(ps),
                            _ => panic!("vector * odd k-vector should be a motor"),
                        };
                        let t2 = match rhs_g3.reverse_geo_kvector(self) {
                            Even(m) => m,
                            KVec(Zero(s)) => Motor::from(s),
                            KVec(Two(bv)) => Motor::from(bv),
                            KVec(Four(ps)) => Motor::from(ps),
                            _ => panic!("trivector * odd k-vector should be a motor"),
                        };
                        Versor::from(t1 + t2)
                    }
                    _ => panic!("Anything mod 2 should be 0 or 1"),
                }
            }
            KVec(kv) => match kv {
                Zero(s) => Versor::from(self * s),
                One(v) => v.reverse_geo_kvector(self),
                Two(bv) => bv.reverse_geo_kvector(self),
                Three(tv) => tv.reverse_geo_kvector(self),
                Four(ps) => self.inner(ps).into(),
            },
        }
    }

    fn normalize(self) -> Self {
        if K == 3 && self[0] != 0.0 {
            self / self[0]
        } else {
            self / self.magnitude()
        }
    }

    fn inverse(self) -> Option<Self> {
        match K {
            1 | 3 => {
                if self.is_ideal() {
                    return None;
                }
                Some(self.reverse() / self.magnitude().powi(2))
            }
            2 => {
                let mut square = match self * self {
                    Versor::Even(m) => m,
                    Versor::KVec(AnyKVector::Zero(s)) => Motor::from(s),
                    Versor::KVec(AnyKVector::Four(ps)) => Motor::from(ps),
                    _ => panic!("Square of bivector should be scalar + pseudoscalar"),
                };
                square[7] = -square[7];
                match (self * square) / square[0].powi(2) {
                    Versor::KVec(AnyKVector::Two(bv)) => {
                        Some(Self::from(bv.components.extract::<0, N>()))
                    }
                    _ => panic!("Inverse of a bivector should be a bivector"),
                }
            }
            _ => panic!("Attempt to invert kvector of invalid grade: {K}"),
        }
    }
}

impl<const K: u8, const N: usize> SingleGrade for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn outer<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        use AnyKVector::*;
        let rev_mult = if K == 1 { -1.0 } else { 1.0 };
        match rhs.into() {
            Zero(s) => (self * s).into(),
            One(v) => v.outer_kvector(self.grade_involution()),
            Two(bv) => bv.outer_kvector(self),
            Three(tv) => tv.outer_kvector(self * rev_mult),
            Four(_) => 0.0.into(),
        }
    }

    #[inline]
    fn inner<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        use AnyKVector::*;
        let rev_mult = if K == 1 { -1.0 } else { 1.0 };
        match rhs.into() {
            Zero(s) => (self * s).into(),
            One(v) => v.inner_kvector(-self.grade_involution()),
            Two(bv) => bv.inner_kvector(self * rev_mult),
            Three(tv) => tv.inner_kvector(self),
            Four(ps) => {
                let result = -Self::from(Simd::load_or(self.real_part(), Simd::splat(0.0)))
                    .dual()
                    .grade_involution()
                    * ps.0;
                match result {
                    Versor::KVec(kv) => kv,
                    _ => panic!("Negative dual of real part of a k-vector should be a k-vector"),
                }
            }
        }
    }

    #[inline]
    fn assert<T: SingleGrade + 'static>(self) -> T {
        use std::any::Any;
        let Some(res) = (&self as &dyn Any).downcast_ref::<T>() else {
            panic!("Single-grade assert failed");
        };
        *res
    }

    #[inline]
    fn scale(self, scale: Trivector) -> Self {
        match K {
            1 => {
                let mut res = self;
                res[3] = self[3] * (self | scale.undual().assert::<Vector>()).assert::<Scalar>()
                    / (self[0] + self[1] + self[2]);
                res
            }
            2 => {
                let origin = ((e123 | self) * self)
                    .normalize()
                    .assert::<Trivector>()
                    .scale(scale);
                let forwards = Trivector::from([0.0, -self[2], -self[1], -self[0]]);
                ((origin + forwards.scale(scale)) & origin).assert::<Self>()
            }
            3 => {
                let new = self.components.extract::<0, 4>() * scale.components;
                Self::from(Simd::load_or(new.as_array(), Simd::from([0.0; N])))
            }
            _ => panic!("Invalid K-vector grade"),
        }
    }
}

impl<const K: u8, const N: usize> KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
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
    const fn reverse_multiplier() -> Scalar {
        match K {
            2 => -1.0,
            3 => -1.0,
            _ => 1.0,
        }
    }

    #[inline]
    pub fn real_part(&self) -> &[f32] {
        &self[0..Self::ideal_index()]
    }

    #[inline]
    pub fn ideal_part(&self) -> &[f32] {
        &self[Self::ideal_index()..N]
    }

    pub fn inner_kvector<const G: u8, const D: usize>(self, rhs: KVector<G, D>) -> AnyKVector
    where
        LaneCount<D>: SupportedLaneCount,
    {
        fn inner_vector_trivector(v: Vector, tv: Trivector) -> Bivector {
            let t1 = simd_swizzle!(v.components, [2, 1, 0, 2, 0, 1]);
            let t2 = simd_swizzle!(tv.components, [0, 0, 0, 2, 3, 1]);
            let t3 = Simd::from([0.0, 0.0, 0.0, v[1], v[2], v[0]]);
            let t4 = Simd::from([0.0, 0.0, 0.0, tv[3], tv[1], tv[2]]);
            Bivector::from(t1 * t2 - t3 * t4).into()
        }
        fn inner_vector_bivector(v: Vector, bv: Bivector) -> Vector {
            let mut t1 = simd_swizzle!(v.components, [2, 0, 1, 0]);
            t1[3] = -t1[3];
            let t2 = simd_swizzle!(bv.components, [1, 0, 2, 3]);
            let t3 = simd_swizzle!(v.components, [1, 2, 0, 1]);
            let t4 = simd_swizzle!(bv.components, [0, 2, 1, 4]);
            let t5 = Simd::from([0.0, 0.0, 0.0, v[2] * bv[5]]);
            Vector::from(t1 * t2 - t3 * t4 - t5)
        }
        fn inner_bivector_trivector(bv: Bivector, tv: Trivector) -> Vector {
            let t1 = Simd::from([0.0, 0.0, 0.0, bv[0] * tv[3]]);
            let mut t2 = simd_swizzle!(bv.components, [2, 1, 0, 1]);
            t2[3] = -t2[3];
            let t3 = simd_swizzle!(tv.components, [0, 0, 0, 2]);
            let t4 = Simd::from([0.0, 0.0, 0.0, bv[2] * tv[1]]);
            Vector::from(t1 - t2 * t3 + t4)
        }
        match K as i8 - G as i8 {
            0 => {
                let Some(rhs) = (&rhs as &dyn Any).downcast_ref::<Self>() else {
                    return 0.0.into();
                };
                (self.components * rhs.components * Simd::splat(Self::reverse_multiplier()))
                    [0..Self::ideal_index()]
                    .iter()
                    .fold(0.0, |acc, f| acc + f)
                    .into()
            }
            1 => match K {
                2 => {
                    let Some(v) = (&rhs as &dyn Any).downcast_ref::<Vector>() else {
                        panic!(
                            "Right-hand-side of inner product where K - G = 1 and K = 2 should be a vector"
                        );
                    };
                    let Some(bv) = (&self as &dyn Any).downcast_ref::<Bivector>() else {
                        panic!(
                            "Left-hand-side of inner product where K - G = 1 and K = 2 should be a bivector"
                        );
                    };
                    (-inner_vector_bivector(*v, *bv)).into()
                }
                3 => {
                    let Some(bv) = (&rhs as &dyn Any).downcast_ref::<Bivector>() else {
                        panic!(
                            "Right-hand-side of inner product where K - G = 1 and K = 3 should be a bivector"
                        );
                    };
                    let Some(tv) = (&self as &dyn Any).downcast_ref::<Trivector>() else {
                        panic!(
                            "Left-hand-side of inner product where K - G = 1 and K = 3 should be a trivector"
                        );
                    };
                    inner_bivector_trivector(*bv, *tv).into()
                }
                _ => panic!(
                    "Left-hand-side of inner product with output grade 1 where K > G should be of grade 2 or 3"
                ),
            },
            -1 => match K {
                1 => {
                    let Some(v) = (&self as &dyn Any).downcast_ref::<Vector>() else {
                        panic!(
                            "Left-hand-side of inner product where K - G = -1 and K = 1 should be a vector"
                        );
                    };
                    let Some(bv) = (&rhs as &dyn Any).downcast_ref::<Bivector>() else {
                        panic!(
                            "Left-hand-side of inner product where K - G = -1 and K = 1 should be a bivector"
                        );
                    };
                    (inner_vector_bivector(*v, *bv)).into()
                }
                2 => {
                    let Some(bv) = (&self as &dyn Any).downcast_ref::<Bivector>() else {
                        panic!(
                            "Left-hand-side of inner product where K - G = -1 and K = 2 should be a bivector"
                        );
                    };
                    let Some(tv) = (&rhs as &dyn Any).downcast_ref::<Trivector>() else {
                        panic!(
                            "Right-hand-side of inner product where K - G = -1 and K = 2 should be a trivector"
                        );
                    };
                    inner_bivector_trivector(*bv, *tv).into()
                }
                _ => panic!(
                    "Left-hand-side of inner product with output grade 1 where K < G should be of grade 1 or 2"
                ),
            },
            2 => {
                let Some(v) = (&rhs as &dyn Any).downcast_ref::<Vector>() else {
                    panic!("Right-hand-side of inner product where K - G = 2 should be a vector");
                };
                let Some(tv) = (&self as &dyn Any).downcast_ref::<Trivector>() else {
                    panic!("Left-hand-side of inner product where K - G = 2 should be a trivector");
                };
                inner_vector_trivector(*v, *tv).into()
            }
            -2 => {
                let Some(v) = (&self as &dyn Any).downcast_ref::<Vector>() else {
                    panic!("Left-hand-side of inner product where K - G = -2 should be a vector");
                };
                let Some(tv) = (&rhs as &dyn Any).downcast_ref::<Trivector>() else {
                    panic!(
                        "Right-hand-side of inner product where K - G = -2 should be a trivector"
                    );
                };
                inner_vector_trivector(*v, *tv).into()
            }
            _ => panic!("Absolute difference of the grade of KVectors should be <= 2"),
        }
    }

    pub fn outer_kvector<const G: u8, const D: usize>(self, rhs: KVector<G, D>) -> AnyKVector
    where
        LaneCount<D>: SupportedLaneCount,
    {
        let result: AnyKVector = match K + G {
            2 => {
                const I1: [usize; 6] = [0, 2, 1, 3, 3, 3];
                const I2: [usize; 6] = [1, 0, 2, 0, 1, 2];
                let t1 = simd_swizzle!(self.components, I1);
                let t2 = simd_swizzle!(rhs.components, I2);
                let t3 = simd_swizzle!(self.components, I2);
                let t4 = simd_swizzle!(rhs.components, I1);
                Bivector::from(t1 * t2 - t3 * t4).into()
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
                Trivector::from(t1 + t2 + t3 + t4 + t5 + t6).into()
            }
            4 => match G as i8 - K as i8 {
                0 => Pseudoscalar(
                    self[0] * rhs[5]
                        + self[1] * rhs[4]
                        + self[2] * rhs[3]
                        + self[3] * rhs[2]
                        + self[4] * rhs[1]
                        + self[5] * rhs[0],
                )
                .into(),
                -2 | 2 => Pseudoscalar(
                    self.e(0b1000) * rhs.e(0b0111)
                        + self.e(0b0100) * rhs.e(0b1011)
                        + self.e(0b0010) * rhs.e(0b1101)
                        + self.e(0b0001) * rhs.e(0b1110)
                        - self.e(0b0111) * rhs.e(0b1000)
                        - self.e(0b1011) * rhs.e(0b0100)
                        - self.e(0b1101) * rhs.e(0b0010)
                        - self.e(0b1110) * rhs.e(0b0001),
                )
                .into(),
                _ => panic!(
                    "Absolute difference of grades of KVectors whose grades sum to 4 should be either 0 or 2"
                ),
            },
            _ => 0.0.into(),
        };
        if result.zero() {
            AnyKVector::Zero(0.0)
        } else {
            result
        }
    }

    #[inline]
    pub fn reverse_geo_kvector<const G: u8, const D: usize>(self, rhs: KVector<G, D>) -> Versor
    where
        LaneCount<D>: SupportedLaneCount,
    {
        self.reverse().geo_kvector(rhs.reverse()).reverse()
    }

    pub fn geo_kvector<const G: u8, const D: usize>(self, rhs: KVector<G, D>) -> Versor
    where
        LaneCount<D>: SupportedLaneCount,
    {
        use AnyKVector::*;
        match (K as i8 - G as i8).abs() % 2 {
            0 => match K + G {
                2 => {
                    let s = match self | rhs {
                        Zero(s) => s,
                        _ => panic!(
                            "First term of geometric product between vectors should be a scalar"
                        ),
                    };
                    let bv = match self ^ rhs {
                        Two(bv) => {
                            if !bv.zero() {
                                bv
                            } else {
                                return Versor::from(Zero(s));
                            }
                        }
                        Zero(0.0) => return Versor::from(Zero(s)),
                        _ => panic!(
                            "Second term of geometric product between vectors should be a bivector"
                        ),
                    };
                    Versor::from(Motor::from((s, bv, Pseudoscalar(0.0))))
                }
                _ => match K == G {
                    false => {
                        let bv = match self | rhs {
                            Two(bv) => bv,
                            Zero(0.0) => return Versor::from(self ^ rhs),
                            _ => panic!(
                                "First term of geometric product producing a motor where K != G should be a bivector"
                            ),
                        };
                        let ps = match self ^ rhs {
                            Four(ps) => ps,
                            Zero(0.0) => return Versor::from(Two(bv)),
                            _ => panic!(
                                "Second term of geometric product producing a motor where K != G should be a pseudoscalar"
                            ),
                        };
                        Versor::from(Motor::from((0.0, bv, ps)))
                    }
                    true => {
                        let s = match self | rhs {
                            Zero(s) => s,
                            _ => panic!(
                                "First term of geometric product producing a motor where K == G should be a scalar"
                            ),
                        };
                        let bv = {
                            match rhs.into() {
                                Two(rhs) => {
                                    let t1 = simd_swizzle!(self.components, [1, 2, 0, 0, 2, 1]);
                                    let t2 = simd_swizzle!(rhs.components, [2, 0, 1, 4, 5, 3]);
                                    let t3 = simd_swizzle!(self.components, [2, 0, 1, 1, 0, 2]);
                                    let t4 = simd_swizzle!(rhs.components, [1, 2, 0, 5, 3, 4]);
                                    let t5 = Simd::from([
                                        0.0,
                                        0.0,
                                        0.0,
                                        self[5] * rhs[1],
                                        self[3] * rhs[0],
                                        self[4] * rhs[2],
                                    ]);
                                    let t6 = Simd::from([
                                        0.0,
                                        0.0,
                                        0.0,
                                        self[4] * rhs[0],
                                        self[5] * rhs[2],
                                        self[3] * rhs[1],
                                    ]);
                                    Bivector::from(t1 * t2 - t3 * t4 + t5 - t6)
                                }
                                Three(rhs) => {
                                    let t1 =
                                        self.components.extract::<1, 3>() * Simd::splat(rhs[0]);
                                    let t2 =
                                        rhs.components.extract::<1, 3>() * Simd::splat(self[0]);
                                    let result =
                                        (t1 - t2).resize::<6>(0.0).rotate_elements_right::<3>();
                                    Bivector::from(result)
                                }
                                _ => panic!(
                                    "Geometric product producing a motor where K == G should be between same-grade elements"
                                ),
                            }
                        };
                        let ps = match self ^ rhs {
                            Four(ps) => ps,
                            Zero(0.0) => {
                                return Versor::from(Motor::from((s, bv, Pseudoscalar(0.0))));
                            }
                            _ => panic!(
                                "Last term of geometric product between bivectors should be a pseudoscalar"
                            ),
                        };
                        Versor::from(Motor::from((s, bv, ps)))
                    }
                },
            },
            1 => {
                let v = match self | rhs {
                    One(v) => v,
                    Zero(0.0) => return Versor::from(self ^ rhs),
                    _ => panic!(
                        "First term of geometric product producing an odd versor should be a vector"
                    ),
                };
                match K + G {
                    3 => {
                        let tv = match self ^ rhs {
                            Three(tv) => tv,
                            Zero(0.0) => return Versor::from(v),
                            _ => panic!(
                                "Second term of geometric product producing an odd versor should be a trivector"
                            ),
                        };
                        Versor::from(OddVersor::from((v, tv)))
                    }
                    5 => {
                        let (bv, tv, sign_correction) = match (N, D) {
                            (4, 6) => (
                                (&rhs as &dyn Any)
                                    .downcast_ref::<Bivector>()
                                    .expect("rhs should be a bivector")
                                    .components,
                                (&self as &dyn Any)
                                    .downcast_ref::<Trivector>()
                                    .expect("lhs should be a trivector")
                                    .components,
                                -1.0,
                            ),
                            (6, 4) => (
                                (&self as &dyn Any)
                                    .downcast_ref::<Bivector>()
                                    .expect("rhs should be a bivector")
                                    .components,
                                (&rhs as &dyn Any)
                                    .downcast_ref::<Trivector>()
                                    .expect("lhs should be a trivector")
                                    .components,
                                1.0,
                            ),
                            _ => {
                                panic!("Invalid geometric product arguments: {:?}/{:?}", self, rhs)
                            }
                        };
                        let t1 = simd_swizzle!(bv, [0, 2, 1]);
                        let t2 = simd_swizzle!(tv, [2, 3, 1]);
                        let t3 = simd_swizzle!(bv, [1, 0, 2]);
                        let t4 = simd_swizzle!(tv, [3, 1, 2]);
                        let t5 = simd_swizzle!(bv, [3, 4, 5]);
                        let t6 = simd_swizzle!(tv, [0, 0, 0]);
                        let r = t1 * t2 - t3 * t4 - t5 * t6;
                        let triv_part = Trivector::from([0.0, r[0], r[1], r[2]]) * sign_correction;
                        Versor::from(OddVersor::from((v, triv_part)))
                    }
                    _ => panic!("Can't multiply {K}-vector and {G}-vector"),
                }
            }
            _ => panic!("Anything mod 2 should be either 0 or 1"),
        }
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
            panic!("v1 ^ v2 was not a bivector; got {:?}", v1 ^ v2);
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

    #[test]
    fn inner_vector_vector() {
        let v1 = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let v2 = Vector::from([4.0, 3.0, 2.0, 1.0]);
        assert_eq!(v1 | v2, 16.0.into());
    }

    #[test]
    fn inner_bivector_bivector() {
        let bv1 = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let bv2 = Bivector::from([6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        assert_eq!(bv1 | bv2, (-28.0).into());
    }

    #[test]
    fn inner_vector_bivector() {
        let bv = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let v = Vector::from([4.0, 3.0, 2.0, 1.0]);
        let expected = AnyKVector::One(Vector::from([-1.0, 2.0, -1.0, 43.0]));
        assert_eq!(bv | v, expected);
        assert_eq!(v | bv, -expected);
    }

    #[test]
    fn inner_bivector_trivector() {
        let bv = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let tv = Trivector::from([4.0, 3.0, 2.0, 1.0]);
        let expected = AnyKVector::One(Vector::from([-12.0, -8.0, -4.0, 14.0]));
        assert_eq!(bv | tv, expected);
        assert_eq!(tv | bv, expected);
    }

    #[test]
    fn inner_vector_trivector() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let tv = Trivector::from([4.0, 3.0, 2.0, 1.0]);
        let expected: AnyKVector = Bivector::from([12.0, 8.0, 4.0, 4.0, -8.0, 4.0]).into();
        assert_eq!(v | tv, expected);
        assert_eq!(tv | v, expected);
    }

    #[test]
    fn geo_vector_vector() {
        let v1 = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let v2 = Vector::from([4.0, 3.0, 2.0, 1.0]);
        let expected = Versor::Even(Motor::from([16.0, -5.0, 10.0, -5.0, 15.0, 10.0, 5.0, 0.0]));
        assert_eq!(v1 * v2, expected);
        assert_eq!((v2 * v1).reverse(), expected);
    }

    #[test]
    fn geo_vector_bivector() {
        let v = Vector::from([2.0, 1.0, 2.0, 1.0]);
        let bv = Bivector::from([-5.0, 10.0, -5.0, 15.0, 10.0, 5.0]);
        let expected = Versor::from(OddVersor::from([
            25.0, 0.0, -25.0, -50.0, -10.0, -10.0, 10.0, 10.0,
        ]));
        assert_eq!(v * bv, expected);
        assert_eq!(bv * v, -expected.reverse());
    }

    #[test]
    fn geo_vector_trivector() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let tv = Trivector::from([4.0, 3.0, 2.0, 1.0]);
        let expected = Versor::Even(Motor::from([0.0, 12.0, 8.0, 4.0, 4.0, -8.0, 4.0, 26.0]));
        assert_eq!(v * tv, expected);
        assert_eq!(tv * v, -expected.reverse());
    }

    #[test]
    fn geo_bivector_bivector() {
        let bv1 = Bivector::from([-5.0, 10.0, -5.0, 15.0, 10.0, 5.0]);
        let bv2 = Bivector::from([12.0, 8.0, 4.0, 4.0, -8.0, 4.0]);
        let expected = Versor::Even(Motor::from([
            0.0, 80.0, -40.0, -160.0, -80.0, 160.0, -80.0, 80.0,
        ]));
        assert_eq!(bv1 * bv2, expected);
        assert_eq!(bv2 * bv1, expected.reverse());
    }

    #[test]
    fn geo_bivector_trivector() {
        let bv = Bivector::from([-5.0, 10.0, -5.0, 15.0, 10.0, 5.0]);
        let tv = Trivector::from([4.0, 3.0, 2.0, 1.0]);
        let expected = Versor::from(OddVersor::from([
            20.0, -40.0, 20.0, 0.0, 0.0, -80.0, -30.0, 20.0,
        ]));
        assert_eq!(bv * tv, expected);
        assert_eq!(tv * bv, expected.reverse());
    }

    #[test]
    fn geo_trivector_trivector() {
        let tv1 = Trivector::from([1.0, 2.0, 3.0, 4.0]);
        let tv2 = Trivector::from([4.0, 3.0, 2.0, 1.0]);
        let expected = Versor::Even(Motor::from([-4.0, 0.0, 0.0, 0.0, 5.0, 10.0, 15.0, 0.0]));
        assert_eq!(tv1 * tv2, expected);
        assert_eq!(tv2 * tv1, expected.reverse());
    }

    #[test]
    fn vector_times_inverse_1() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v * v.inverse().unwrap(), Versor::KVec(1.0.into()));
    }

    #[test]
    fn trivector_times_inverse_1() {
        let tv = Trivector::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(tv * tv.inverse().unwrap(), Versor::KVec(1.0.into()));
    }

    #[test]
    fn bivector_times_inverse_1() {
        let bv1 = Bivector::from([-5.0, 10.0, -5.0, 15.0, 10.0, 5.0]);
        let bv2 = Bivector::from([-5.0, 10.0, -5.0, 13.0, 10.0, 5.0]);
        assert_eq!(
            bv1.geo(bv1.inverse().unwrap()).snap(),
            Versor::KVec(1.0.into())
        );
        assert_eq!(
            bv2.geo(bv2.inverse().unwrap()).snap(),
            Versor::KVec(1.0.into())
        );
    }

    #[test]
    fn vector_normalized_mag_1() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        assert!(float_eq(v.normalize().magnitude(), 1.0));
    }

    #[test]
    fn bivector_normalized_mag_1() {
        let bv = Bivector::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert!(float_eq(bv.normalize().magnitude(), 1.0));
    }
}
