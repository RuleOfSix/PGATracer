use crate::pga_3::*;
use crate::util::float_eq;
use std::ops::{Add, Div, Mul, Neg, Shr, Sub};
use std::ops::{Index, IndexMut};
use std::simd::{Simd, simd_swizzle};
use std::slice::SliceIndex;

pub enum Transformation {
    Rotation {
        axis: Bivector,
        angle: f32,
    },
    Translation {
        direction: Trivector,
    },
    Screw {
        axis: Bivector,
        angle: f32,
        distance: f32,
    },
}

impl Transformation {
    pub fn rotation(axis: Bivector, angle: f32) -> Self {
        Transformation::Rotation {
            axis: axis.normalize(),
            angle,
        }
    }
    pub fn translation(dir: Trivector) -> Self {
        Transformation::Translation { direction: dir }
    }
    pub fn screw(axis: Bivector, angle: f32, distance: f32) -> Self {
        Transformation::Screw {
            axis: axis.normalize(),
            angle,
            distance,
        }
    }
}

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

impl From<Transformation> for Motor {
    #[inline]
    fn from(t: Transformation) -> Self {
        use AnyKVector::*;
        use Transformation::*;
        use Versor::*;
        match t {
            Rotation { axis, angle } => axis.mul(-angle / 2.0).exp(),
            Translation { direction } => {
                let KVec(Two(bv)) = -(Vector::from([0.0, 0.0, 0.0, 1.0]) * direction.dual()) / 2.0
                else {
                    panic!("Line at infinity should be a line");
                };
                Self::from((1.0, bv, Pseudoscalar(0.0)))
            }
            Screw {
                axis,
                angle,
                distance,
            } => {
                let KVec(Two(bv_i)) = axis * Pseudoscalar(1.0) * distance / 2.0 else {
                    panic!("Line at infinity must be a bivector");
                };
                match bv_i.exp() * axis.mul(-angle / 2.0).exp() {
                    Even(m) => m,
                    _ => panic!("Screw motion should be a motor"),
                }
            }
        }
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
        self.components
            .as_array()
            .iter()
            .enumerate()
            .fold(true, |acc, (i, e)| acc && float_eq(*e, other[i]))
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

impl<T: SingleGrade + NonScalar> Shr<T> for Motor {
    type Output = AnyKVector;
    fn shr(self, rhs: T) -> Self::Output {
        self.sandwich(rhs)
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
        self[0..4] == [0.0; 4]
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

    fn normalize(self) -> Self {
        use AnyKVector::*;
        use Versor::*;
        let squared = match self * self.reverse() {
            Even(m) => m,
            KVec(Zero(s)) => Motor::from(s),
            KVec(Four(ps)) => Motor::from(ps),
            _ => panic!("Motor squared should be scalar + pseudoscalar"),
        };
        let s = 1.0 / squared[0].sqrt();
        let ps = -squared[7] / (2.0 * squared[0].sqrt().powi(3));
        match self * Motor::from([s, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, ps]) {
            Even(m) => m,
            KVec(Zero(s)) => Motor::from(s),
            KVec(Four(ps)) => Motor::from(ps),
            _ => panic!("Motor normalized should be a motor"),
        }
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
                let t2 = match self_g2.geo(m) {
                    Even(m) => m,
                    KVec(kv) => match kv {
                        AnyKVector::Zero(s) => Motor::from(s),
                        AnyKVector::Two(bv) => Motor::from(bv),
                        AnyKVector::Four(ps) => Motor::from(ps),
                        _ => panic!("Bivector * Motor should be motor"),
                    },
                    _ => panic!("Bivector * Motor should be motor"),
                };
                let t3 = match self_g4.geo(m) {
                    Even(m) => m,
                    KVec(kv) => match kv {
                        AnyKVector::Zero(s) => Motor::from(s),
                        AnyKVector::Two(bv) => Motor::from(bv),
                        AnyKVector::Four(ps) => Motor::from(ps),
                        _ => panic!("Pseudoscalar * Motor should be motor"),
                    },
                    _ => panic!("Pseudoscalar * Motor should be motor"),
                };
                Even(t1 + t2 + t3)
            }
            Odd(ov) => {
                let t1 = ov * self_g0;
                let t2 = match self_g2.geo(ov) {
                    Odd(ov) => ov,
                    KVec(kv) => match kv {
                        AnyKVector::One(v) => OddVersor::from(v),
                        AnyKVector::Three(tv) => OddVersor::from(tv),
                        _ => panic!("Bivector * odd versor should be odd versor"),
                    },
                    _ => panic!("Bivector * odd versor should be odd versor"),
                };
                let t3 = match self_g4.geo(ov) {
                    Odd(ov) => ov,
                    KVec(kv) => match kv {
                        AnyKVector::One(v) => OddVersor::from(v),
                        AnyKVector::Three(tv) => OddVersor::from(tv),
                        _ => panic!("Pseudoscalar * odd versor should be odd versor"),
                    },
                    _ => panic!("Pseudoscalar * odd versor should be odd versor"),
                };
                Odd(t1 + t2 + t3)
            }
        }
    }
}

impl Motor {
    pub fn sandwich<T: SingleGrade + NonScalar>(self, rhs: T) -> AnyKVector {
        match self.reverse() * rhs * self {
            Versor::KVec(kv) => kv,
            _ => panic!("Sandwich of k-vector should be a k-vector"),
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

    #[test]
    fn motor_normalize() {
        let m = Motor::from([-1.0, 0.0, 0.0, 1.0, -2.0, 0.0, 0.0, 2.0]).normalize();
        assert!(float_eq(m.magnitude(), 1.0));
        assert_eq!(m.inverse().unwrap(), m.reverse());
    }

    #[test]
    fn rotate_plane() {
        use std::f32::consts::PI;
        let p = Vector::from([1.0, 0.0, 0.0, 0.0]);
        let r = Transformation::rotation(Bivector::from([0.0, 1.0, 0.0, 0.0, 0.0, 0.0]), PI / 4.0);
        let expected = Vector::from([1.0, 0.0, 1.0, 0.0]).normalize();
        let m = Motor::from(r);
        assert_eq!(m.reverse() * p * m, Versor::from(expected));
        assert_eq!(m.sandwich(p), expected.into());
        assert_eq!(m >> p, expected.into());
    }

    #[test]
    fn translate_plane() {
        let p = Vector::from([1.0, 0.0, 0.0, 0.0]);
        let dir = Trivector::direction(5.0, 0.0, 0.0);
        let t = Transformation::translation(dir);
        let expected = Vector::from([1.0, 0.0, 0.0, 5.0]).normalize();
        assert_eq!(Motor::from(t) >> p, expected.into());
    }

    #[test]
    fn screw_line() {
        use std::f32::consts::PI;
        let x_axis = Bivector::from([0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
        let z_axis = Bivector::from([1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let s = Transformation::screw(z_axis, PI / 4.0, 5.0);
        let result = (Motor::from(s) >> x_axis).normalize();
        let Versor::KVec(AnyKVector::Three(p1)) =
            (Trivector::from([1.0, 0.0, 0.0, 0.0]) | result) * result
        else {
            panic!("Point projected onto line should be a point");
        };
        let p2 = p1 + {
            let Versor::KVec(AnyKVector::Three(bv)) = Vector::from([0.0, 0.0, 0.0, 1.0]) * result
            else {
                panic!("e0 * bivector should be trivector");
            };
            bv
        };
        println!("P1: {:?}\nP2: {:?}", p1, p2);
        assert_eq!(
            result,
            Bivector::from([
                0.0,
                -0.7071066499,
                0.7071068883,
                -3.5355329514,
                -3.5355343819,
                0.0
            ])
            .into()
        );
    }
}
