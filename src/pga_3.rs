pub use kvector::*;
use std::marker::Sized;
use std::ops::{Div, Mul, Neg};
use std::simd::{LaneCount, SupportedLaneCount};
pub use versor::*;

pub trait Multivector:
    Sized + Neg + Mul<Scalar, Output = Self> + Div<Scalar, Output = Self> + Into<Versor> + Copy + Clone
{
    fn e(&self, basis: u8) -> f32;
    fn grade(&self, g: u8) -> AnyKVector;
    fn highest_grade(&self) -> u8;
    fn zero(&self) -> bool;
    fn is_ideal(&self) -> bool;
    fn eucl_norm(&self) -> f32 {
        self.reverse().geo(*self).e(0b0000).sqrt()
    }
    fn ideal_norm(&self) -> f32 {
        self.dual().eucl_norm()
    }
    fn magnitude(&self) -> f32 {
        if self.is_ideal() {
            self.ideal_norm()
        } else {
            self.eucl_norm()
        }
    }

    fn reverse(&self) -> Self;
    fn grade_involution(&self) -> Self;
    fn dual(self) -> Versor;
    fn undual(self) -> Versor;
    fn geo<T: Multivector>(self, rhs: T) -> Versor;

    fn normalize(self) -> Self {
        self / self.magnitude()
    }

    fn inverse(self) -> Option<Self> {
        if self.is_ideal() {
            return None;
        }
        Some(self.reverse() / self.magnitude().powi(2))
    }
}

pub trait SingleGrade: Multivector + Into<AnyKVector> {
    fn outer<T: SingleGrade>(self, rhs: T) -> AnyKVector;
    fn inner<T: SingleGrade>(self, rhs: T) -> AnyKVector;
    fn assert<T: SingleGrade + 'static>(self) -> T;
    fn scale(self, scale: Trivector) -> Self;

    fn regressive<T: SingleGrade>(self, rhs: T) -> AnyKVector {
        let Versor::KVec(d1) = self.dual() else {
            panic!("Dual of k-vector should be a k-vector");
        };
        let Versor::KVec(d2) = rhs.dual() else {
            panic!("Dual of k-vector should be a k-vector");
        };
        let Versor::KVec(result) = (d1 ^ d2).undual() else {
            panic!("Undual of outer product of k-vectors should be a k-vector");
        };
        result
    }
}

pub trait NonScalar {}
impl<const K: u8, const N: usize> NonScalar for KVector<K, N> where LaneCount<N>: SupportedLaneCount {}
impl NonScalar for Pseudoscalar {}
impl NonScalar for AnyKVector {}
impl NonScalar for Versor {}
impl NonScalar for OddVersor {}
impl NonScalar for Motor {}

mod kvector;
mod versor;
