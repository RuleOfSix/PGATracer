use crate::pga_3::*;
use crate::util::float_eq;
use AnyKVector::*;
use std::any::Any;
use std::ops::{BitAnd, BitOr, BitXor, Div, Mul, Neg};
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

impl<T> Mul<T> for AnyKVector
where
    T: Multivector,
    T: NonScalar,
{
    type Output = Versor;

    fn mul(self, rhs: T) -> Self::Output {
        self.geo(rhs)
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

impl<T: SingleGrade> BitXor<T> for AnyKVector {
    type Output = AnyKVector;

    #[inline]
    fn bitxor(self, rhs: T) -> Self::Output {
        self.outer(rhs)
    }
}

impl<T: SingleGrade> BitAnd<T> for AnyKVector {
    type Output = AnyKVector;

    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        self.regressive(rhs)
    }
}

impl<T: SingleGrade> BitOr<T> for AnyKVector {
    type Output = AnyKVector;
    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        self.inner(rhs)
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

impl Multivector for AnyKVector {
    #[inline]
    fn reverse(&self) -> Self {
        match self {
            Zero(s) => s.reverse().into(),
            One(v) => v.reverse().into(),
            Two(bv) => bv.reverse().into(),
            Three(tv) => tv.reverse().into(),
            Four(ps) => ps.reverse().into(),
        }
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        match self {
            Zero(s) => s.grade_involution().into(),
            One(v) => v.grade_involution().into(),
            Two(bv) => bv.grade_involution().into(),
            Three(tv) => tv.grade_involution().into(),
            Four(ps) => ps.grade_involution().into(),
        }
    }

    #[inline]
    fn e(&self, basis: u8) -> f32 {
        match self {
            Zero(s) => s.e(basis),
            One(v) => v.e(basis),
            Two(bv) => bv.e(basis),
            Three(tv) => tv.e(basis),
            Four(ps) => ps.e(basis),
        }
    }

    #[inline]
    fn grade(&self, g: u8) -> Self {
        match self {
            Zero(s) => s.grade(g),
            One(v) => v.grade(g),
            Two(bv) => bv.grade(g),
            Three(tv) => tv.grade(g),
            Four(ps) => ps.grade(g),
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        match self {
            Zero(_) => 0,
            One(_) => 1,
            Two(_) => 2,
            Three(_) => 3,
            Four(_) => 4,
        }
    }

    #[inline]
    fn zero(&self) -> bool {
        match self {
            Zero(s) => s.zero(),
            One(v) => v.zero(),
            Two(bv) => bv.zero(),
            Three(tv) => tv.zero(),
            Four(ps) => ps.zero(),
        }
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        match self {
            Zero(s) => s.is_ideal(),
            One(v) => v.is_ideal(),
            Two(bv) => bv.is_ideal(),
            Three(tv) => tv.is_ideal(),
            Four(ps) => ps.is_ideal(),
        }
    }

    #[inline]
    fn dual(self) -> Versor {
        match self {
            Zero(s) => {
                if s != 0.0 {
                    Pseudoscalar(s).into()
                } else {
                    self.into()
                }
            }
            One(v) => v.dual().into(),
            Two(bv) => bv.dual().into(),
            Three(tv) => tv.dual().into(),
            Four(ps) => Zero(ps.0).into(),
        }
    }

    #[inline]
    fn undual(self) -> Versor {
        match self {
            One(v) => v.undual().into(),
            Three(tv) => tv.undual().into(),
            _ => self.dual().into(),
        }
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        match self {
            Zero(s) => (rhs * s).into(),
            One(v) => v.geo(rhs),
            Two(bv) => bv.geo(rhs),
            Three(tv) => tv.geo(rhs),
            Four(ps) => ps.geo(rhs),
        }
    }
}

impl SingleGrade for AnyKVector {
    #[inline]
    fn outer<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        self.outer_self(rhs.into())
    }

    #[inline]
    fn inner<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        match self {
            Zero(s) => (rhs * s).into(),
            One(v) => v.inner(rhs),
            Two(bv) => bv.inner(rhs),
            Three(tv) => tv.inner(rhs),
            Four(ps) => ps.inner(rhs),
        }
    }
}

impl AnyKVector {
    #[inline]
    pub fn outer_self(self, rhs: Self) -> Self {
        match self {
            Zero(s) => rhs * s,
            One(v) => v.outer(rhs),
            Two(bv) => bv.outer(rhs),
            Three(tv) => tv.outer(rhs),
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

#[macro_export]
macro_rules! type_of {
    (Zero) => {
        Scalar
    };
    (One) => {
        Vector
    };
    (Two) => {
        Bivector
    };
    (Three) => {
        Trivector
    };
    (Four) => {
        Pseudoscalar
    };
}

#[macro_export]
macro_rules! anykvec_sum {
    ( {$variant:tt} $($anykv:expr),*) => {{
        use AnyKVector::*;
        let mut result: Option<type_of!($variant)> = None;
        for anykvec in [ $($anykv, )* ] {
            if let $variant(v) = anykvec {
                result = match result {
                    Some(r) => Some(r + v),
                    None => None,
                };
            };
        }
        match result {
            Some(r) => $variant(r),
            None => AnyKVector::Zero(0.0),
        }
    }};
}
