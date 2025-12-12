use std::cmp::PartialEq;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
use std::simd::{LaneCount, Simd, SupportedLaneCount};
use std::slice::SliceIndex;

pub mod trivector;

#[derive(Debug, Copy, Clone)]
pub struct KVector<const K: u8, const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    components: Simd<f32, N>,
}

impl<Idx, const K: u8, const N: usize> Index<Idx> for KVector<K, N>
where
    Idx: SliceIndex<[f32]>,
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.components[index]
    }
}

impl<Idx, const K: u8, const N: usize> IndexMut<Idx> for KVector<K, N>
where
    Idx: SliceIndex<[f32], Output = f32>,
    LaneCount<N>: SupportedLaneCount,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.components[index]
    }
}

impl<const K: u8, const N: usize> PartialEq for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl<const K: u8, const N: usize> Add for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            components: self.components + rhs.components,
        }
    }
}

impl<const K: u8, const N: usize> Sub for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            components: self.components - rhs.components,
        }
    }
}

impl<const K: u8, const N: usize> Neg for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            components: -self.components,
        }
    }
}

impl<const K: u8, const N: usize> Mul<f32> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            components: self.components * Simd::splat(other),
        }
    }
}

impl<const K: u8, const N: usize> Div<f32> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            components: self.components / Simd::splat(other),
        }
    }
}

impl<const K: u8, const N: usize> From<[f32; N]> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn from(value: [f32; N]) -> Self {
        Self {
            components: Simd::from(value),
        }
    }
}

impl<const K: u8, const N: usize> From<Simd<f32, N>> for KVector<K, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn from(value: Simd<f32, N>) -> Self {
        Self { components: value }
    }
}
