use super::Scalar;
use crate::pga_3::Multivector;
use crate::util::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Pseudoscalar(pub f32);

impl PartialEq for Pseudoscalar {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.0, other.0)
    }
}

impl Neg for Pseudoscalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Pseudoscalar(-self.0)
    }
}

impl Add for Pseudoscalar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pseudoscalar(self.0 + rhs.0)
    }
}

impl Sub for Pseudoscalar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Pseudoscalar(self.0 - rhs.0)
    }
}

impl Mul<Scalar> for Pseudoscalar {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Pseudoscalar(self.0 * rhs)
    }
}

impl Div<Scalar> for Pseudoscalar {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self::Output {
        Pseudoscalar(self.0 / rhs)
    }
}

impl Multivector for Pseudoscalar {
    fn reverse(&self) -> Self {
        *self
    }
    fn grade_involution(&self) -> Self {
        *self
    }
}
