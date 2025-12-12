use crate::util::*;
use std::simd::Simd;
use std::slice::SliceIndex;

use super::KVector;

type Trivector = KVector<3, 4>;

impl Trivector {
    fn point(x: f32, y: f32, z: f32) -> Self {
        Self::from([1.0, -x, -y, -z])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_point() {
        let point = Trivector::point(4.3, -4.2, 3.1);
        assert!(float_eq(point[0], 4.3));
        assert!(float_eq(point[1], -4.2));
        assert!(float_eq(point[2], 3.1));
        assert!(float_eq(point[3], 1.0));
        assert!(point.is_finite());
    }

    #[test]
    fn basic_dir() {
        let dir = Trivector::new(4.3, -4.2, 3.1, 0.0);
        assert!(float_eq(dir[0], 4.3));
        assert!(float_eq(dir[1], -4.2));
        assert!(float_eq(dir[2], 3.1));
        assert!(float_eq(dir[3], 0.0));
        assert!(!dir.is_finite());
    }

    #[test]
    fn point_new() {
        let point = Trivector::new_point(4.0, -4.0, 3.0);
        assert_eq!(point[0..4], [4.0, -4.0, 3.0, 1.0]);
    }

    #[test]
    fn dir_new() {
        let dir = Trivector::new_dir(4.0, -4.0, 3.0);
        assert_eq!(dir[0..4], [4.0, -4.0, 3.0, 0.0]);
    }

    #[test]
    fn trivector_addition() {
        let a1 = Trivector::new_point(3.0, -2.0, 5.0);
        let a2 = Trivector::new_dir(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Trivector::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_points() {
        let p1 = Trivector::new_point(3.0, 2.0, 1.0);
        let p2 = Trivector::new_point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Trivector::new_dir(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_dir_from_point() {
        let p = Trivector::new_point(3.0, 2.0, 1.0);
        let p_at_inf = Trivector::new_dir(5.0, 6.0, 7.0);
        assert_eq!(p - p_at_inf, Trivector::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_dirs() {
        let d1 = Trivector::new_dir(3.0, 2.0, 1.0);
        let d2 = Trivector::new_dir(5.0, 6.0, 7.0);
        assert_eq!(d1 - d2, Trivector::new_dir(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negation() {
        let a = Trivector::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Trivector::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn scalar_multiplication() {
        let a = Trivector::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Trivector::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn scalar_division() {
        let a = Trivector::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Trivector::new(0.5, -1.0, 1.5, -2.0));
        assert_eq!(a / 2.0, a * 0.5);
    }

    #[test]
    fn magnitude() {
        assert_eq!(Trivector::new(1.0, 0.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Trivector::new(0.0, 1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Trivector::new(0.0, 0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Trivector::new(0.0, 0.0, 0.0, 1.0).magnitude(), 1.0);
        assert!(float_eq(
            Trivector::new(1.0, 2.0, 3.0, 4.0).magnitude(),
            30_f32.sqrt()
        ));
        assert!(float_eq(
            Trivector::new(-1.0, -2.0, -3.0, -4.0).magnitude(),
            30_f32.sqrt()
        ));
    }

    #[test]
    fn normalize() {
        let a = Trivector::new_dir(4.0, 0.0, 0.0);
        assert_eq!(a.normalize(), Trivector::new_dir(1.0, 0.0, 0.0));

        let b = Trivector::new(1.0, 2.0, 3.0, 4.0);
        let b_mag = f32::sqrt(30.0);
        let b_norm = b.normalize();
        assert_eq!(
            b_norm,
            Trivector::new(1.0 / b_mag, 2.0 / b_mag, 3.0 / b_mag, 4.0 / b_mag)
        );
        assert!(float_eq(b_norm.magnitude(), 1.0));
    }
}
