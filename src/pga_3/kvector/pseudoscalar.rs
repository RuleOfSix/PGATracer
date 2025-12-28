use crate::pga_3::*;
use crate::util::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Pseudoscalar(pub f32);

pub const e0123: Pseudoscalar = Pseudoscalar(1.0);

impl PartialEq for Pseudoscalar {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.0, other.0)
    }
}

impl Neg for Pseudoscalar {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Pseudoscalar(-self.0)
    }
}

impl Add for Pseudoscalar {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Pseudoscalar(self.0 + rhs.0)
    }
}

impl Sub for Pseudoscalar {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Pseudoscalar(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Pseudoscalar {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Scalar) -> Self::Output {
        Pseudoscalar(self.0 * rhs)
    }
}

impl<T: Multivector + NonScalar> Mul<T> for Pseudoscalar {
    type Output = Versor;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.geo(rhs)
    }
}

impl Div<Scalar> for Pseudoscalar {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Scalar) -> Self::Output {
        Pseudoscalar(self.0 / rhs)
    }
}

impl Multivector for Pseudoscalar {
    #[inline]
    fn reverse(&self) -> Self {
        *self
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        *self
    }

    #[inline]
    fn e(&self, basis: u8) -> f32 {
        match basis {
            0b1111 => self.0,
            _ => 0.0,
        }
    }

    #[inline]
    fn grade(&self, g: u8) -> AnyKVector {
        match g {
            4 => AnyKVector::Four(*self),
            _ => AnyKVector::Zero(0.0),
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        4
    }

    #[inline]
    fn zero(&self) -> bool {
        float_eq(self.0, 0.0)
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        true
    }

    #[inline]
    fn dual(self) -> Versor {
        self.0.into()
    }

    #[inline]
    fn undual(self) -> Versor {
        self.dual()
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        use Versor::*;
        match rhs.into() {
            KVec(kv) => self.inner(kv).into(),
            Even(m) => {
                let ps = Pseudoscalar(m[0] * self.0);
                let bv = Bivector::from([0.0, 0.0, 0.0, -m[3], -m[2], -m[1]]) * self.0;
                Even(Motor::from((0.0, bv, ps)))
            }
            Odd(ov) => {
                let v = Vector::from([-ov[4], 0.0, 0.0, 0.0]);
                let tv = Trivector::from([0.0, ov[0], ov[1], ov[2]]);
                Odd(OddVersor::from((v, tv)))
            }
        }
    }
}

impl SingleGrade for Pseudoscalar {
    #[inline]
    fn outer<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        for g in 1..=4 {
            if !rhs.grade(g).zero() {
                return 0.0.into();
            }
        }
        (self * rhs.e(0b0000)).into()
    }

    #[inline]
    fn inner<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        use AnyKVector::*;
        match rhs.into() {
            Zero(s) => (self * s).into(),
            One(v) => -v.inner(self),
            Two(bv) => bv.inner(self),
            Three(tv) => -tv.inner(self),
            Four(_) => 0.0.into(),
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
    fn scale(self, _: Trivector) -> Self {
        panic!("Cannot scale pseudoscalar");
    }
}
