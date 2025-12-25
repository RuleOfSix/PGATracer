use crate::pga_3::*;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{Index, IndexMut};
use std::simd::{Simd, simd_swizzle};
use std::slice::SliceIndex;

#[derive(Copy, Clone, Debug)]
pub struct Motor {
    components: Simd<f32, 8>,
}

impl From<[f32; 8]> for Motor {
    #[inline]
    fn from(cs: [f32; 8]) -> Self {
        Motor {
            components: Simd::from(cs),
        }
    }
}

impl From<Simd<f32, 8>> for Motor {
    #[inline]
    fn from(cs: Simd<f32, 8>) -> Self {
        Motor { components: cs }
    }
}

impl From<Scalar> for Motor {
    #[inline]
    fn from(s: Scalar) -> Self {
        Self::from([s, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
    }
}

impl From<Bivector> for Motor {
    #[inline]
    fn from(bv: Bivector) -> Self {
        Self::from([0.0, bv[0], bv[1], bv[2], bv[3], bv[4], bv[5], 0.0])
    }
}

impl From<Pseudoscalar> for Motor {
    #[inline]
    fn from(ps: Pseudoscalar) -> Self {
        Self::from([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ps.0])
    }
}

impl From<(Scalar, Bivector, Pseudoscalar)> for Motor {
    #[inline]
    fn from(cs: (Scalar, Bivector, Pseudoscalar)) -> Self {
        Self::from([
            cs.0, cs.1[0], cs.1[1], cs.1[2], cs.1[3], cs.1[4], cs.1[5], cs.2.0,
        ])
    }
}

impl<Idx: SliceIndex<[f32]>> Index<Idx> for Motor {
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.components[index]
    }
}

impl<Idx: SliceIndex<[f32]>> IndexMut<Idx> for Motor {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl PartialEq for Motor {
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl Neg for Motor {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Motor {
            components: -self.components,
        }
    }
}

impl Add for Motor {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Motor {
            components: self.components + rhs.components,
        }
    }
}

impl Sub for Motor {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Motor {
            components: self.components - rhs.components,
        }
    }
}

impl Mul<Scalar> for Motor {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Motor {
            components: self.components * Simd::splat(rhs),
        }
    }
}

impl<T: Multivector + NonScalar> Mul<T> for Motor {
    type Output = Versor;
    fn mul(self, rhs: T) -> Self::Output {
        self.geo(rhs)
    }
}

impl Div<Scalar> for Motor {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self::Output {
        Motor {
            components: self.components / Simd::splat(rhs),
        }
    }
}

impl Multivector for Motor {
    #[inline]
    fn e(&self, basis: u8) -> f32 {
        match basis {
            0b0000 => self[0],
            0b0110 => self[1],
            0b0101 => self[2],
            0b0011 => self[3],
            0b1100 => self[4],
            0b1010 => self[5],
            0b1001 => self[6],
            0b1111 => self[7],
            _ => 0.0,
        }
    }

    #[inline]
    fn grade(&self, g: u8) -> AnyKVector {
        match g {
            0 => self[0].into(),
            2 => Bivector::from(self.components.extract::<1, 6>()).into(),
            4 => Pseudoscalar(self[7]).into(),
            _ => 0.0.into(),
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        if self[7] != 0.0 {
            return 4;
        }
        if self[1..7] != [0.0; 6] {
            return 2;
        }
        0
    }

    #[inline]
    fn reverse(&self) -> Self {
        Motor::from([
            self[0], -self[1], -self[2], -self[3], -self[4], -self[5], -self[6], self[7],
        ])
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        *self
    }

    #[inline]
    fn zero(&self) -> bool {
        self.components == Simd::splat(0.0)
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        self[4..8] == [0.0; 4]
    }

    #[inline]
    fn dual(self) -> Versor {
        Versor::from(Motor::from(simd_swizzle!(
            self.components,
            [0, 6, 5, 4, 3, 2, 1, 7]
        )))
    }

    #[inline]
    fn undual(self) -> Versor {
        self.dual()
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        use Versor::*;
        let self_g0 = self[0];
        let AnyKVector::Two(self_g2) = self.grade(2) else {
            panic!("Grade 2 part of motor should be bivector");
        };
        let self_g4 = Pseudoscalar(self[7]);
        match rhs.into() {
            KVec(kv) => kv.reverse().geo(self.reverse()).reverse(),
            Even(m) => {
                let t1 = m * self_g0;
                let Even(t2) = self_g2.geo(m) else {
                    panic!("Bivector * Motor should be motor");
                };
                let Even(t3) = self_g4.geo(m) else {
                    panic!("Pseudoscalar * Motor should be motor");
                };
                Even(t1 + t2 + t3)
            }
            Odd(ov) => {
                let t1 = ov * self_g0;
                let Odd(t2) = self_g2.geo(ov) else {
                    panic!("Bivector * odd versor should be odd versor");
                };
                let Odd(t3) = self_g4.geo(ov) else {
                    panic!("Pseudoscalar * odd versor should be odd versor");
                };
                Odd(t1 + t2 + t3)
            }
        }
    }
}

impl Motor {
    pub fn sqrt(self) -> Self {
        let mut t1 = self.clone();
        t1[0] += 1.0;
        let denom = 2.0 * (1.0 + self[0]);
        let lhs = t1 / denom.sqrt();
        let rhs = Self::from((
            1.0,
            Bivector::from(Simd::splat(0.0)),
            Pseudoscalar(self.e(0b1111) / denom),
        ));
        match lhs * rhs {
            Versor::Even(m) => m,
            _ => panic!("Square root of motor should be a motor"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn geo_motor_bivector() {
        let m = Motor::from([-1.0, 0.0, 0.0, 1.0, -2.0, 0.0, 0.0, 2.0]);
        let bv = Bivector::from([-5.0, 10.0, -5.0, 15.0, 10.0, 5.0]);
        let expected = Versor::Even(Motor::from([
            5.0, -5.0, -15.0, 5.0, -5.0, -15.0, 15.0, 25.0,
        ]));
        let expected_reverse =
            Versor::Even(Motor::from([5.0, 15.0, -5.0, 5.0, -5.0, -45.0, -5.0, 25.0]));

        assert_eq!(m * bv, expected);
        assert_eq!(bv * m, expected_reverse);
    }

    #[test]
    fn geo_motor_vector() {
        let m = Motor::from([-1.0, 0.0, 0.0, 1.0, -2.0, 0.0, 0.0, 2.0]);
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let expected = Versor::Odd(OddVersor::from([
            -1.0, 1.0, -5.0, -6.0, 1.0, -6.0, -10.0, -2.0,
        ]));
        let expected_reverse = Versor::Odd(OddVersor::from([
            -1.0, -5.0, -1.0, -2.0, 1.0, -2.0, -2.0, 10.0,
        ]));

        assert_eq!(m * v, expected);
        assert_eq!(v * m, expected_reverse);
    }
}
