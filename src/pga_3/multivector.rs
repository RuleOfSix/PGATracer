use super::Bivector;
use super::Pseudoscalar;
use super::Scalar;
use super::Trivector;
use super::Vector;
use std::marker::Sized;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Multivector: Sized + Add + Sub + Neg + Mul<Scalar> + Div<Scalar> {
    fn reverse(&self) -> Self;
    fn grade_involution(&self) -> Self;
}

pub enum Versor {
    Even(Motor),
    Odd(OddVersor),
}

pub struct OddVersor {
    g1: Vector,
    g3: Trivector,
}

pub struct Motor {
    g0: Scalar,
    g2: Bivector,
    g4: Pseudoscalar,
}
