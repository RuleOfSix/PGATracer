use crate::pga_3::*;
use crate::util::float_eq;

pub type Scalar = f32;

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
}
