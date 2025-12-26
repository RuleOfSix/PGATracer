use crate::pga_3::*;
use crate::util::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{Index, IndexMut};
use std::simd::{Simd, simd_swizzle};
use std::slice::SliceIndex;

#[derive(Clone, Copy, Debug)]
pub struct OddVersor {
    components: Simd<f32, 8>,
}
impl From<[f32; 8]> for OddVersor {
    #[inline]
    fn from(cs: [f32; 8]) -> Self {
        OddVersor {
            components: Simd::from(cs),
        }
    }
}

impl From<Simd<f32, 8>> for OddVersor {
    #[inline]
    fn from(cs: Simd<f32, 8>) -> Self {
        OddVersor { components: cs }
    }
}

impl From<Vector> for OddVersor {
    #[inline]
    fn from(v: Vector) -> Self {
        Self::from([v[0], v[1], v[2], v[3], 0.0, 0.0, 0.0, 0.0])
    }
}

impl From<Trivector> for OddVersor {
    fn from(tv: Trivector) -> Self {
        Self::from([0.0, 0.0, 0.0, 0.0, tv[0], tv[1], tv[2], tv[3]])
    }
}

impl From<(Vector, Trivector)> for OddVersor {
    #[inline]
    fn from(cs: (Vector, Trivector)) -> Self {
        OddVersor {
            components: Simd::from([
                cs.0[0], cs.0[1], cs.0[2], cs.0[3], cs.1[0], cs.1[1], cs.1[2], cs.1[3],
            ]),
        }
    }
}

impl<Idx: SliceIndex<[f32]>> Index<Idx> for OddVersor {
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.components[index]
    }
}

impl<Idx: SliceIndex<[f32]>> IndexMut<Idx> for OddVersor {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl PartialEq for OddVersor {
    fn eq(&self, other: &Self) -> bool {
        self.components
            .as_array()
            .iter()
            .enumerate()
            .fold(true, |acc, (i, e)| acc && float_eq(*e, other[i]))
    }
}

impl Neg for OddVersor {
    type Output = Self;

    fn neg(self) -> Self::Output {
        OddVersor {
            components: -self.components,
        }
    }
}

impl Add for OddVersor {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        OddVersor {
            components: self.components + rhs.components,
        }
    }
}

impl Sub for OddVersor {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        OddVersor {
            components: self.components - rhs.components,
        }
    }
}

impl Mul<Scalar> for OddVersor {
    type Output = Self;
    fn mul(self, rhs: Scalar) -> Self::Output {
        OddVersor {
            components: self.components * Simd::splat(rhs),
        }
    }
}

impl<T: Multivector + NonScalar> Mul<T> for OddVersor {
    type Output = Versor;
    fn mul(self, rhs: T) -> Self::Output {
        self.geo(rhs)
    }
}

impl Div<Scalar> for OddVersor {
    type Output = Self;
    fn div(self, rhs: Scalar) -> Self::Output {
        OddVersor {
            components: self.components / Simd::splat(rhs),
        }
    }
}

impl Multivector for OddVersor {
    #[inline]
    fn e(&self, basis: u8) -> f32 {
        match basis {
            0b0100 => self[0],
            0b0010 => self[1],
            0b0001 => self[2],
            0b1000 => self[3],
            0b0111 => self[4],
            0b1011 => self[5],
            0b1101 => self[6],
            0b1110 => self[7],
            _ => 0.0,
        }
    }

    #[inline]
    fn grade(&self, g: u8) -> AnyKVector {
        match g {
            1 => Vector::from(simd_swizzle!(self.components, [0, 1, 2, 3])).into(),
            3 => Trivector::from(simd_swizzle!(self.components, [4, 5, 6, 7])).into(),
            _ => 0.0.into(),
        }
    }

    #[inline]
    fn highest_grade(&self) -> u8 {
        if self[4..8] != [0.0; 4] {
            return 3;
        }
        if self[0..4] != [0.0; 4] {
            return 1;
        }
        0
    }

    #[inline]
    fn reverse(&self) -> Self {
        OddVersor::from([
            self[0], self[1], self[2], self[3], -self[4], -self[5], -self[6], -self[7],
        ])
    }

    #[inline]
    fn grade_involution(&self) -> Self {
        -*self
    }

    #[inline]
    fn zero(&self) -> bool {
        self.components == Simd::splat(0.0)
    }

    #[inline]
    fn is_ideal(&self) -> bool {
        self[3] == 0.0 && self[5..8] == [0.0; 3]
    }

    #[inline]
    fn dual(self) -> Versor {
        Versor::from(OddVersor::from([
            -self[5], -self[6], -self[7], -self[4], self[3], self[0], self[1], self[2],
        ]))
    }

    #[inline]
    fn undual(self) -> Versor {
        -self.dual()
    }

    #[inline]
    fn geo<T: Multivector>(self, rhs: T) -> Versor {
        use Versor::*;
        let AnyKVector::One(self_g1) = self.grade(1) else {
            panic!("Grade 1 part of odd versor should be vector");
        };
        let AnyKVector::Three(self_g3) = self.grade(3) else {
            panic!("Grade 3 part of odd versor should be trivector");
        };
        match rhs.into() {
            KVec(kv) => kv.reverse().geo(self.reverse()).reverse(),
            Even(m) => {
                let Odd(t1) = self_g1.geo(m) else {
                    panic!("Vector * Motor should be odd versor");
                };
                let Odd(t2) = self_g3.geo(m) else {
                    panic!("Trivector * Motor should be odd versor");
                };
                Odd(t1 + t2)
            }
            Odd(ov) => {
                let Even(t1) = self_g1.geo(ov) else {
                    panic!("Vector * odd versor should be motor");
                };
                let Even(t2) = self_g3.geo(ov) else {
                    panic!("Trivector * odd versor should be motor");
                };
                Even(t1 + t2)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn geo_oddversor_vector() {
        let ov = OddVersor::from([-1.0, 1.0, -5.0, -6.0, 1.0, -6.0, -10.0, -2.0]);
        let v = Vector::from([1.0, 2.0, 3.0, 4.0]);
        let expected = Versor::Even(Motor::from([-14.0, 0.0, 0.0, 14.0, -28.0, 0.0, 0.0, 28.0]));
        let expected_reverse = Versor::Even(Motor::from([
            -14.0, 6.0, 4.0, -12.0, -24.0, 32.0, -4.0, -28.0,
        ]));
        assert_eq!(ov * v, expected);
        assert_eq!(v * ov, expected_reverse);
    }

    #[test]
    fn geo_oddversor_bivector() {
        let ov = OddVersor::from([-1.0, 1.0, -5.0, -6.0, 1.0, -6.0, -10.0, -2.0]);
        let bv = Bivector::from([-5.0, 10.0, -5.0, 15.0, 10.0, 5.0]);
        let expected = Versor::Odd(OddVersor::from([
            -40.0, -30.0, 10.0, -30.0, 40.0, -30.0, 20.0, 60.0,
        ]));
        let expected_reverse = Versor::Odd(OddVersor::from([
            50.0, 10.0, 0.0, -90.0, 40.0, 80.0, -40.0, -170.0,
        ]));

        assert_eq!(ov * bv, expected);
        assert_eq!(bv * ov, expected_reverse);
    }
}
