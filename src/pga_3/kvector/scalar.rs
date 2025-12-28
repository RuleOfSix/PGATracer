use crate::pga_3::*;
use crate::util::float_eq;
use std::simd::{LaneCount, SupportedLaneCount};

pub type Scalar = f32;

impl<const K: u8, const N: usize> Mul<KVector<K, N>> for Scalar
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = KVector<K, N>;
    #[inline]
    fn mul(self, other: KVector<K, N>) -> Self::Output {
        other * self
    }
}

impl Mul<Pseudoscalar> for Scalar {
    type Output = Pseudoscalar;
    #[inline]
    fn mul(self, other: Pseudoscalar) -> Self::Output {
        other * self
    }
}

impl Mul<AnyKVector> for Scalar {
    type Output = AnyKVector;
    #[inline]
    fn mul(self, other: AnyKVector) -> Self::Output {
        other * self
    }
}

impl Mul<Motor> for Scalar {
    type Output = Motor;
    #[inline]
    fn mul(self, other: Motor) -> Self::Output {
        other * self
    }
}

impl Mul<OddVersor> for Scalar {
    type Output = OddVersor;
    #[inline]
    fn mul(self, other: OddVersor) -> Self::Output {
        other * self
    }
}

impl Mul<Versor> for Scalar {
    type Output = Versor;
    #[inline]
    fn mul(self, other: Versor) -> Self::Output {
        other * self
    }
}

impl Multivector for Scalar {
    #[inline]
    fn e(&self, basis: u8) -> f32 {
        match basis {
            0b0000 => *self,
            _ => 0.0,
        }
    }

    #[inline]
    fn grade(&self, g: u8) -> AnyKVector {
        if g == 0 {
            AnyKVector::Zero(*self)
        } else {
            AnyKVector::Zero(0.0)
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        0
    }

    #[inline]
    fn zero(&self) -> bool {
        float_eq(*self, 0.0)
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        false
    }

    #[inline]
    fn reverse(&self) -> Self {
        *self
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        *self
    }

    #[inline]
    fn dual(self) -> Versor {
        Versor::from(Pseudoscalar(self))
    }

    #[inline]
    fn undual(self) -> Versor {
        self.dual()
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        (rhs * self).into()
    }
}

impl SingleGrade for Scalar {
    #[inline]
    fn outer<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        (rhs * self).into()
    }

    #[inline]
    fn inner<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        (rhs * self).into()
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
        panic!("Cannot scale scalar");
    }
}
