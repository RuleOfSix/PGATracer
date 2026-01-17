use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        const COLOR_EPSILON: f32 = 0.001;
        f32::abs(self.red - other.red) < COLOR_EPSILON
            && f32::abs(self.green - other.green) < COLOR_EPSILON
            && f32::abs(self.blue - other.blue) < COLOR_EPSILON
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Color {
    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue }
    }

    pub fn similar_to(&self, other: Self, difference_threshold: f32) -> bool {
        f32::abs(self.red - other.red) < difference_threshold
            && f32::abs(self.green - other.green) < difference_threshold
            && f32::abs(self.blue - other.blue) < difference_threshold
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_creation() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn color_mul_f32() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_mul_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
